//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1038/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1038<F: Float>(t21906: F, t3403: F, t11369: F, t11372: F, t14702: F, t14766: F, t18203: F, t18219: F, t18229: F, t18494: F, t18505: F, t18512: F, t21739: F, t21741: F, t21747: F, t21751: F) -> (F, F) {
    let t21907 = t21906 * t3403;
    let t21922 = -t11369 - F::cast_from(0.16557e0_f64) * t18512 + F::cast_from(0.20128333333333333333e0_f64) * t18203 - F::cast_from(0.60385000000000000001e0_f64) * t18219 - F::cast_from(0.30192500000000000001e0_f64) * t18229 + F::cast_from(0.5519e-1_f64) * t18494 - F::cast_from(0.33114e0_f64) * t18505 - F::cast_from(0.3883875e1_f64) * t21739 + F::cast_from(0.247573125e0_f64) * t21741 - t11372 + F::cast_from(0.40256666666666666668e0_f64) * t14702 + F::cast_from(0.27595e0_f64) * t14766 - F::cast_from(0.82785e-1_f64) * t21747 + F::cast_from(0.49671e0_f64) * t21751;
    (t21907, t21922)
}
