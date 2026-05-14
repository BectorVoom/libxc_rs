//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1318/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1318<F: Float>(t4292: F, t94: F, t1513: F, t665: F, t93: F, t2178: F, t3813: F, t1310: F, t8273: F, t2175: F, t2289: F, t2339: F, t625: F) -> (F, F, F, F, F, F, F) {
    let t27126 = t94 * t4292;
    let t28036 = t1513 * t665;
    let t28219 = t93 * t4292;
    let t31013 = t3813 * t2178;
    let t31016 = t1310 * t8273;
    let t31026 = 11.0 / 9.0 * t2289 * t2175;
    let t31027 = t625 * t2339;
    (t27126, t28036, t28219, t31013, t31016, t31026, t31027)
}
