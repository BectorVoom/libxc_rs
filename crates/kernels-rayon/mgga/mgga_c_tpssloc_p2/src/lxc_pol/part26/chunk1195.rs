//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1195/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1195(t40590: f64, t68: f64, t3700: f64, t2751: f64, t10047: f64, t225: f64, t9587: f64, t9585: f64, t10108: f64, t257: f64, t252: f64, t9957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = 1.0_f64 / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0_f64 / t40771;
    let t40852 = t10047 * t225;
    let t40870 = t9587 * t225;
    let t40875 = t9585 * t225;
    let t40889 = 1.0_f64 / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40909 = t252 * t9957;
    (t40591, t40611, t40772, t40852, t40870, t40875, t40890, t40909)
}
