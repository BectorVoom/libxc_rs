//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1040/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1040(t225: f64, t387: f64, t6768: f64, t345: f64, t1065: f64, t8396: f64, t10165: f64, t8391: f64, t990: f64, t6726: f64, t8384: f64, t1948: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30800 = t6768 * t225 * t387;
    let t30801 = t345 * t30800;
    let t30804 = t8396 * t1065;
    let t30805 = t10165 * t30804;
    let t30808 = t990 * t8391;
    let t30813 = 0.40372756094140390856e-3_f64 * t6726 * t8384;
    let t30816 = t1948 * sigma0;
    (t30800, t30801, t30805, t30808, t30813, t30816)
}
