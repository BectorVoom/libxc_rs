//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 800/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk800(t13854: f64, t13887: f64, t13903: f64, t13912: f64, t12846: f64, t12850: f64, t12851: f64, t12853: f64, t12854: f64, t13004: f64, t13006: f64, t13243: f64, t13761: f64, t13762: f64, t13767: f64, t13839: f64, t13841: f64, t748: f64) -> (f64, f64) {
    let t13914 = t13854 + t13887 + t13903 + t13912;
    let t13916 = -t13914 * t748 + t12846 + t12850 + t12851 - t12853 + t12854 + t13004 - t13006 + t13243 - t13761 + t13762 - t13767 + 2.0_f64 * t13839 - t13841;
    (t13914, t13916)
}
