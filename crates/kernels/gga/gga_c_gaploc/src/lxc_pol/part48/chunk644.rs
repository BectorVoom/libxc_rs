//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 644/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk644<F: Float>(t13525: F, t836: F, t568: F, t314: F, t313: F, t11757: F, t955: F, t11765: F, t1645: F, t3451: F, t13538: F, t1445: F, t1457: F, t13506: F, t6060: F, t13073: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13668 = t836 * t13525;
    let t13669 = t568 * t13668;
    let t13672 = t314 * t13525;
    let t13673 = t313 * t13672;
    let t13679 = 0.35750489951850426669e0 * t955 * t11757;
    let t13681 = 0.35750489951850426669e0 * t955 * t11765;
    let t13682 = t1645 * t3451;
    let t13685 = t1445 * t13538;
    let t13688 = t1457 * t13538;
    let t13691 = t1457 * t13506;
    let t13693 = 0.21450293971110256001e1 * t6060 * t13691;
    let t13695 = 0.17875244975925213335e0 * t13073;
    (t13668, t13669, t13672, t13673, t13679, t13681, t13682, t13685, t13688, t13691, t13693, t13695)
}
