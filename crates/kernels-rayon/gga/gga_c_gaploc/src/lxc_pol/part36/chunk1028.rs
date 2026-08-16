//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1028/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1028(t12845: f64, t12847: f64, t12849: f64, t12850: f64, t12851: f64, t12853: f64, t12855: f64, t12858: f64, t12861: f64, t12864: f64, t13002: f64, t13248: f64, t42488: f64, t44251: f64, t7: f64) -> f64 {
    let tv4rhosigma31 = t12845 - t12847 + t12849 - t12850 - t12851 + t12853 - t12855 - t12858 + t12861 + t12864 - t13002 + t13248 + t7 * (t42488 + t44251);
    tv4rhosigma31
}
