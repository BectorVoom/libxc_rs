//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2854/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2854<F: Float>(t190: F, t706: F, t76397: F, t40092: F, t40094: F, t14330: F, t18305: F, t4181: F, t61201: F, t157: F, t23121: F, t606: F) -> (F, F, F, F, F, F) {
    let t76986 = F::cast_from(4.0_f64) * t706 * t190 * t76397;
    let t76987 = F::cast_from(0.51947577317044391277e2_f64) * t40092;
    let t76988 = F::cast_from(0.35089341735807877242e1_f64) * t40094;
    let t76991 = F::cast_from(72.0_f64) * t14330 * t18305 * t4181;
    let t76992 = F::cast_from(24.0_f64) * t61201;
    let t76995 = F::cast_from(24.0_f64) * t606 * t157 * t23121;
    (t76986, t76987, t76988, t76991, t76992, t76995)
}
