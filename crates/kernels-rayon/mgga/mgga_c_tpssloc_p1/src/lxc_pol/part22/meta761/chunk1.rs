//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2563/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2563(t14850: f64, t18677: f64, t14838: f64, t18680: f64, t15207: f64, t18640: f64, t4802: f64, t4824: f64, t64103: f64, t64292: f64, t71793: f64, t71795: f64, t71797: f64, t71800: f64, t71803: f64, t71806: f64, t71809: f64, t71811: f64, t71814: f64, t71817: f64) -> (f64, f64, f64) {
    let t71819 = 18.0_f64 * t14850 * t18677;
    let t71821 = 12.0_f64 * t14838 * t18680;
    let t71828 = t71793 - t71795 - t71797 - t71800 + t71803 + t71806 + t71809 - t71811 - t71814 - t71817 - t71819 + t71821 - 6.0_f64 * t64292 * t4802 + 0.96491876992155210402e2_f64 * t64103 * t4824 - 6.0_f64 * t15207 * t18640;
    (t71819, t71821, t71828)
}
