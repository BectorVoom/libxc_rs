//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1622/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1622<F: Float>(t50888: F, t62300: F, t50892: F, t50893: F, t77047: F, t50901: F, t40076: F, t40079: F, t40184: F, t40194: F, t40198: F, t87673: F) -> (F, F, F, F, F, F, F) {
    let t87674 = F::cast_from(0.14035736694323150897e2_f64) * t50888;
    let t87675 = F::new(6.0) * t62300;
    let t87676 = F::new(4.0) * t50892;
    let t87677 = F::cast_from(0.4155806185363551302e3_f64) * t50893;
    let t87678 = F::cast_from(0.23392894490538584828e1_f64) * t77047;
    let t87679 = F::cast_from(0.1301229756036208781e0_f64) * t50901;
    let t87680 = -t40184 + t87673 - t87674 + t87675 + t87676 + t87677 - t87678 + t40076 - t40079 + t40194 + t40198 - t87679;
    (t87674, t87675, t87676, t87677, t87678, t87679, t87680)
}
