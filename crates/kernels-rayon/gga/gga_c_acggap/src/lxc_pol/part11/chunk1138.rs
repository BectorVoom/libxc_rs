//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1138/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1138(t7839: f64, t8962: f64, t31426: f64, t31429: f64, t35581: f64, t35586: f64, t35587: f64, t35591: f64, t35595: f64, t35597: f64, t35599: f64, t35602: f64, t35603: f64, t35609: f64, t35611: f64, t35614: f64, t35617: f64, t35621: f64) -> f64 {
    let t35623 = t7839 * t8962;
    let t35624 = 0.62896184579208304136e-3_f64 * t35623;
    let t35625 = t35581 - t35586 + 0.42874018118069736972e-3_f64 * t35587 - 0.32155513588552302729e-2_f64 * t35591 + t35595 + t35597 + 0.64311027177104605458e-2_f64 * t35599 + t35602 + t35603 - 0.84046875e-1_f64 * t31426 - 11.0_f64 / 96.0_f64 * t31429 + t35609 + t35611 - 0.47172138434406228102e-3_f64 * t35614 - t35617 - 0.7862023072401038017e-3_f64 * t35621 + t35624;
    t35625
}
