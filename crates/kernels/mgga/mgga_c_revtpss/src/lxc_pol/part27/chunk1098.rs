//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1098/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1098<F: Float>(t2014: F, t7238: F, t94358: F, t25090: F, t7235: F, t25803: F, t25802: F, t7312: F, t25866: F, t2470: F, t26049: F, t7284: F, t2453: F, t555: F, t25898: F, t136: F, t137: F, t2022: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t94361 = 9.0 * t2014 * t94358 * t7238;
    let t94369 = 9.0 * t7235 * t25090;
    let t94371 = 3.0 * t7235 * t25803;
    let t94374 = 3.0 * t2014 * t7312 * t25802;
    let t94376 = 18.0 * t7235 * t25866;
    let t94377 = t26049 * t2470;
    let t94378 = t7284 * t94377;
    let t94382 = t2453 * t555;
    let t94383 = t94382 * t25898;
    let t94385 = t2022 * t136 * t137;
    (t94361, t94369, t94371, t94374, t94376, t94377, t94378, t94382, t94383, t94385)
}
