//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 735/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk735(t225: f64, t385: f64, t4930: f64, t1678: f64, t342: f64, t1695: f64, t999: f64, t1079: f64, t1096: f64, t3269: f64, t1086: f64, t1647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4932 = t4930 * t225 * t385;
    let t4935 = t342 * t1678;
    let t4940 = t1695 * t999;
    let t4941 = t1079 * t4940;
    let t4946 = t1695 * t1096;
    let t4947 = t3269 * t4946;
    let t4954 = t1647 * t1086;
    (t4932, t4935, t4940, t4941, t4946, t4947, t4954)
}
