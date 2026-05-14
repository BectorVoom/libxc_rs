//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1070/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1070<F: Float>(t39420: F, t39422: F, t39424: F, t39426: F, t39429: F, t39431: F, t39434: F, t39448: F, t41384: F, t41385: F, t41386: F, t41387: F, t39458: F, t39464: F, t39470: F, t37584: F, t37588: F, t38452: F, t39452: F, t39455: F, t39460: F, t39462: F, t39467: F, t39476: F) -> (F, F) {
    let t41389 = -0.51220160311720645766e0 * t39420 + 0.10975748638225852664e0 * t39422 + 0.21951497276451705328e0 * t39424 - 0.54878743191129263322e-1 * t39426 + 0.31147743054556651237e-1 * t39429 + 0.10975748638225852664e0 * t39431 + 0.17336443480108537126e0 * t39434 + t41384 - t41385 - t41386 + t41387 - 0.20803732176130244552e1 * t39448;
    let t41392 = 0.13869154784086829701e1 * t39458;
    let t41395 = 0.11902492299418487743e0 * t39464;
    let t41397 = 0.28914548798370980346e-3 * t39470;
    let t41401 = 0.34672886960217074252e0 * t39452 - 0.10401866088065122276e1 * t39455 - t41392 + 0.17336443480108537126e0 * t39460 + 0.5200933044032561138e0 * t39462 - t41395 + 0.10401866088065122276e1 * t39467 - t41397 - 0.57131963037208741168e-1 * t37584 - 0.95219938395347901946e-2 * t37588 - t38452 - 0.43663693315433241794e-2 * t39476;
    (t41389, t41401)
}
