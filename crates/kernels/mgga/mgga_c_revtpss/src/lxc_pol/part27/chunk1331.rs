//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1331/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1331<F: Float>(t97525: F, t97537: F, t97550: F, t97565: F, t13240: F, t13244: F, t13247: F, t1461: F, t2170: F, t27102: F, t4162: F, t4165: F, t573: F, t7696: F, t95131: F, t95136: F, t95140: F, t95143: F, t95147: F, t95149: F, t95153: F, t95157: F, t95160: F, t95163: F, t95171: F, t95173: F, t95175: F, param_d: F) -> (F, F) {
    let t97567 = t97525 + t97537 + t97550 + t97565;
    let t97576 = t573 * t97567 * param_d + F::new(6.0) * t13240 * t2170 + F::new(18.0) * t13244 * t2170 + F::new(3.0) * t13247 * t2170 + F::new(9.0) * t1461 * t27102 + F::new(18.0) * t4162 * t7696 + F::new(9.0) * t4165 * t7696 + t95131 + t95136 + t95140 + t95143 + t95147 + t95149 + t95153 + t95157 + t95160 + t95163 + t95171 + t95173 + t95175;
    (t97567, t97576)
}
