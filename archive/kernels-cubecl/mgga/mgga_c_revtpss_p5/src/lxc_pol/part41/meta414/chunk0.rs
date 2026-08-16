//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1465/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1465<F: Float>(t1518: F, t648: F, t4292: F, t94: F, t1513: F, t665: F, t93: F, t5920: F, t1501: F, t2175: F, t2289: F, t2339: F, t625: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27123 = t648 * t1518;
    let t27126 = t94 * t4292;
    let t28036 = t1513 * t665;
    let t28219 = t93 * t4292;
    let t29508 = t94 * t5920;
    let t30138 = t1501 * t1518;
    let t30143 = t93 * t5920;
    let t31026 = F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t2289 * t2175;
    let t31027 = t625 * t2339;
    (t27123, t27126, t28036, t28219, t29508, t30138, t30143, t31026, t31027)
}
