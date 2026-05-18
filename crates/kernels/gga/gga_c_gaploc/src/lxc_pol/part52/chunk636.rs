//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 636/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk636<F: Float>(t11742: F, t11775: F, t11811: F, t11829: F, t11866: F, t11904: F, t11935: F, t11966: F, t3689: F, t555: F) -> (F, F) {
    let t11969 = t11742 + t11775 + t11811 + t11829 + t11866 + t11904 + t11935 + t11966;
    let t11977 = t555 * t3689;
    (t11969, t11977)
}
