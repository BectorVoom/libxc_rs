//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 415/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk415<F: Float>(t431: F, t4359: F, t1038: F, t416: F, t4189: F, t1028: F, t385: F, t381: F, t1156: F, t449: F, t195: F, t452: F, t197: F, t53: F, t57: F, t60: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4361 = 0.35089341735807877242e1 * t431 * t4359;
    let t4363 = t1038 * t416 * t4189;
    let t4365 = 0.51947577317044391277e2 * t431 * t4363;
    let t4366 = t385 * t1028;
    let t4372 = t381 * t1028;
    let t4382 = t449 * t1156;
    let t4388 = 1.0 / t452 / t195;
    let t4389 = t197 * t4388;
    let t4394 = t53 * t53;
    let t4396 = 1.0 / t57 / t4394;
    let t4406 = t60 * t60;
    (t4361, t4365, t4366, t4372, t4382, t4388, t4389, t4396, t4406)
}
