//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1047/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1047<F: Float>(t13894: F, t27843: F, t25413: F, t4391: F, t21487: F, t14943: F, t1580: F, t21621: F, t21626: F, t21668: F, t21672: F, t21675: F, t21900: F, t21902: F, t21908: F, t2328: F, t27810: F, t27814: F, t27819: F, t27834: F, t27840: F, t4397: F, t6453: F, t6459: F, t6482: F, t6502: F, t8332: F) -> (F,) {
    let t27844 = t13894 * t27843;
    let t27847 = t4391 * t25413;
    let t27848 = t21487 * t27847;
    let t27851 = 0.14392630972941853771e0 * t6453 * t2328 - 0.17990788716177317213e-1 * t27810 - 0.35981577432354634426e-1 * t1580 * t27814 - 0.11993859144118211476e-1 * t1580 * t27819 - 0.17990788716177317213e-1 * t6459 * t6502 - 0.39979530480394038252e-2 * t21621 - t21626 + 0.59969295720591057378e-2 * t14943 + 0.59969295720591057377e-2 * t21668 + 0.47975436576472845901e-1 * t21672 + 0.11993859144118211475e-1 * t21675 - 0.17990788716177317213e-1 * t6459 * t6482 + 0.89953943580886586067e-2 * t4397 * t8332 + 0.89953943580886586067e-2 * t1580 * t27834 - 0.47975436576472845901e-1 * t21900 - 0.11993859144118211475e-1 * t21902 + t21908 - 0.71963154864709268855e-1 * t1580 * t27840 + 0.27985671336275826777e-1 * t1580 * t27844 + 0.47975436576472845904e-1 * t1580 * t27848;
    (t27851,)
}
