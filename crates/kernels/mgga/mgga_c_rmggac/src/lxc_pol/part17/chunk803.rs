//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 803/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk803<F: Float>(t2010: F, t38835: F, t8465: F, t2415: F, t38820: F, t7349: F, t2329: F, t38973: F, t118: F, t128: F, t2001: F, t6261: F, t675: F, t10010: F, t2604: F, t2868: F, t39048: F, t45120: F, t45123: F, t45126: F, t45129: F, t45132: F, t45135: F, t45139: F, t45149: F, t6434: F, t6449: F, t665: F, t8994: F, t903: F) -> (F,) {
    let t45152 = t2010 * t8465 * t38835;
    let t45155 = t7349 * t2415 * t38820;
    let t45158 = t38973 * t2329;
    let t45163 = t675 * t2001 * t118 * t128 * t6261;
    let t45165 = 0.81823984962736025184e-1 * t45120 - 0.13637330827122670864e0 * t45123 - 0.54549323308490683456e-1 * t45126 - 0.40911992481368012592e-1 * t45129 + 0.81823984962736025184e-1 * t45132 + 0.40911992481368012592e-1 * t45135 - 0.11974241701863808564e0 * t2868 * t8994 + 0.20455996240684006296e-1 * t45139 + 0.35922725105591425692e0 * t903 * t665 * t6449 + 0.35922725105591425692e0 * t903 * t665 * t6434 + 0.23948483403727617128e0 * t2604 * t10010 - 0.90915538847484472429e-2 * t45149 + 0.72042316457491791906e-3 * t45152 - 0.10248087766267884742e-3 * t45155 + 0.72732431077987577943e-1 * t39048 - 0.27274661654245341728e-1 * t45158 - 0.42564599893297839398e-5 * t45163;
    (t45165,)
}
