//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 650/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk650<F: Float>(t1173: F, t2410: F, t674: F, t1997: F, t2004: F, t2412: F, t2007: F, t1987: F, t1990: F, t457: F, t589: F, t201: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8675 = t2410 * t1173;
    let t8676 = t8675 * t674;
    let t8677 = t8676 * t1997;
    let t8679 = t2412 * t2004;
    let t8681 = t2412 * t2007;
    let t8683 = t2412 * t1987;
    let t8685 = t2412 * t1990;
    let t8687 = t589 * t457;
    let t8688 = t8687 * t201;
    (t8675, t8676, t8677, t8679, t8681, t8683, t8685, t8687, t8688)
}
