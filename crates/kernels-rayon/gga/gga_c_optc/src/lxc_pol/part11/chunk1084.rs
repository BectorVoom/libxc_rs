//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1084/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1084(t1256: f64, t745: f64, t193: f64, t4752: f64, t6654: f64, t2204: f64, t4611: f64, t5068: f64, t7274: f64, t999: f64, t5053: f64, t8393: f64) -> (f64, f64, f64, f64, f64) {
    let t39009 = t745 * t1256;
    let t39030 = t193 * t6654 * t4752;
    let t39066 = t4611 * t2204;
    let t39204 = t999 * t7274 * t5068;
    let t39288 = t5053 * t8393;
    (t39009, t39030, t39066, t39204, t39288)
}
