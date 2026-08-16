//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1508/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1508(t10867: f64, t251: f64, t2777: f64, t2789: f64, t2439: f64, t2435: f64, t2790: f64, t2778: f64, t9303: f64, t871: f64, t9292: f64, t9646: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10952 = t10867 * t251;
    let t10963 = t2777 * t2789;
    let t10964 = t2439 * t10963;
    let t10966 = t2435 * t2790;
    let t10969 = 0.26019841438354088051e-2_f64 * t9303 * t2778;
    let t10971 = 0.17073386770573548589e-1_f64 * t9292 * t871;
    let t10981 = t9646 * t251;
    (t10952, t10963, t10964, t10966, t10969, t10971, t10981)
}
