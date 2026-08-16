//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 676/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk676(t3874: f64, t405: f64, t2396: f64, t758: f64, t3736: f64, t3738: f64, t3742: f64, t3768: f64, t3771: f64, t3827: f64, t3829: f64, t3831: f64, t3835: f64, t3839: f64, t3843: f64) -> (f64, f64, f64, f64) {
    let t3875 = t405 * t3874;
    let t3876 = t3875 * t2396;
    let t3877 = t758 * t3876;
    let t3880 = -t3736 + t3738 - t3742 + t3768 + t3771 + t3827 + t3829 - t3831 + t3835 - t3839 - t3843;
    (t3875, t3876, t3877, t3880)
}
