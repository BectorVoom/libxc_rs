//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1058/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1058(t17030: f64, t232: f64, t6646: f64, t1888: f64, t16815: f64, t2632: f64, t22996: f64, t25224: f64, t7488: f64, t1880: f64, t25: f64, t5664: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28422 = t17030 * t232;
    let t28423 = t6646 * t28422;
    let t28424 = t1888 * t28423;
    let t28426 = t16815 * t2632;
    let t28427 = t22996 * t28426;
    let t28428 = t1888 * t28427;
    let t28439 = t25224 * t7488;
    let t28440 = t1880 * t28439;
    let t28456 = t25 * t5664;
    (t28422, t28423, t28424, t28426, t28427, t28428, t28439, t28440, t28456)
}
