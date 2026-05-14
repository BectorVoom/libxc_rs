//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1153/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1153<F: Float>(t2258: F, t394: F, t9498: F, t1506: F, t5886: F, t32260: F, t6333: F, t2275: F, t3508: F, t1513: F, t5606: F, t2279: F, t3512: F, t6357: F, t9491: F, t32266: F, t9839: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33676 = t2258 * t394;
    let t33677 = t33676 * t9498;
    let t33679 = t5886 * t1506;
    let t33681 = t32260 * t6333;
    let t33683 = t3508 * t2275;
    let t33685 = t5606 * t1513;
    let t33687 = t3512 * t2279;
    let t33689 = t9491 * t6357;
    let t33691 = t32266 * t9839;
    (t33676, t33677, t33679, t33681, t33683, t33685, t33687, t33689, t33691)
}
