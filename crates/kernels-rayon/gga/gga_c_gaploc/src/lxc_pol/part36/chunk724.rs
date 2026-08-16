//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 724/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk724(t13199: f64, t13228: f64, t12847: f64, t12849: f64, t12853: f64, t12855: f64, t12858: f64, t13002: f64, t13004: f64, t13005: f64, t13006: f64, t13166: f64, t331: f64, t748: f64) -> (f64, f64) {
    let t13229 = t13199 + t13228;
    let t13231 = -t748 * t13166 + t13229 * t331 + t12847 - t12849 - t12853 + t12855 + t12858 + t13002 + t13004 - t13005 - 2.0_f64 * t13006;
    (t13229, t13231)
}
