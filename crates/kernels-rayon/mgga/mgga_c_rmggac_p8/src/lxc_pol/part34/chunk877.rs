//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 877/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk877(t13866: f64, t1986: f64, t305: f64, t8614: f64, t14374: f64, t15231: f64, t15344: f64, t70123: f64, t13862: f64, t1616: f64, t3133: f64, t14011: f64, t1654: f64, t3120: f64) -> (f64, f64, f64, f64, f64) {
    let t75685 = t13866 * t1986 * t305 * t8614;
    let t75687 = t14374 * t15231;
    let t75689 = t70123 * t15344;
    let t75692 = t3133 * t13862 * t1616;
    let t75695 = t3120 * t14011 * t1654;
    (t75685, t75687, t75689, t75692, t75695)
}
