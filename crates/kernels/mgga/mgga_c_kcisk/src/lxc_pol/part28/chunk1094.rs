//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1094/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1094<F: Float>(t1775: F, t25046: F, t12263: F, t18443: F, t18453: F, t18456: F, t18458: F, t2013: F, t25024: F, t25027: F, t25030: F, t25034: F, t25038: F, t25042: F, t2023: F, t7718: F) -> (F, F) {
    let t25047 = t1775 * t25046;
    let t25050 = 0.59969295720591057378e-2 * t12263 - 0.59969295720591057377e-2 * t18443 - t18453 - t18456 + 0.79959060960788076504e-2 * t18458 - 0.17990788716177317213e-1 * t25024 - 0.89953943580886586067e-2 * t25027 - 0.11993859144118211476e-1 * t2013 * t25030 - 0.71963154864709268855e-1 * t2013 * t25034 + 0.27985671336275826777e-1 * t2013 * t25038 - 0.47975436576472845904e-1 * t2013 * t25042 + 0.17990788716177317213e-1 * t2013 * t25047;
    let t25052 = t7718 * t2023;
    (t25050, t25052)
}
