//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 670/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk670<F: Float>(t11742: F, t11775: F, t11811: F, t11829: F, t11866: F, t11904: F, t11935: F, t11966: F, t135: F, t139: F, t145: F, t459: F) -> (F, F, F, F) {
    let t11969 = t11742 + t11775 + t11811 + t11829 + t11866 + t11904 + t11935 + t11966;
    let t12380 = F::cast_from(1.0_f64) / t135;
    let t12381 = t12380 * t139;
    let t12383 = t12381 * t145 * t459;
    (t11969, t12380, t12381, t12383)
}
