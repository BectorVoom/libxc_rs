//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1137/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1137(t14228: f64, t4337: f64, t10408: f64, t13510: f64, t13512: f64, t13514: f64, t13517: f64, t13519: f64, t13522: f64, t13524: f64, t13526: f64, t13657: f64, t13661: f64, t13665: f64, t13720: f64, t13722: f64, t13726: f64, t13729: f64, t13731: f64, t13734: f64) -> (f64, f64) {
    let t14234 = t4337 * t14228;
    let t14235 = t10408 * t14234;
    let t14238 = -t13510 + t13512 - t13514 + t13517 + t13519 + t13522 + t13524 + t13526 + t13657 - t13661 + t13665 - t13720 + t13722 + t13726 - t13729 - t13731 + t13734;
    (t14235, t14238)
}
