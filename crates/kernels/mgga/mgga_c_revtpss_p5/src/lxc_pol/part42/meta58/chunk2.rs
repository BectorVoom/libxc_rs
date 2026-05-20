//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 355/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk355<F: Float>(t1122: F, t1145: F, t141: F, t1124: F, t1135: F, t1137: F, t1140: F, t1144: F) -> (F, F, F) {
    let t1146 = t1145 * t1122;
    let t1147 = t141 * t1146;
    let t1149 = F::new(0.1898925e1) * t1135 - t1137 + F::cast_from(0.29896666666666666667e0_f64) * t1124 + F::new(0.3071625e0) * t1140 - t1144 + F::cast_from(0.82156666666666666667e-1_f64) * t1147;
    (t1146, t1147, t1149)
}
