//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 674/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk674<F: Float>(t466: F, t8514: F, t1001: F, t8419: F, t8483: F, t8487: F, t8490: F, t8494: F, t8496: F, t8498: F, t8502: F, t8506: F, t8512: F, t1266: F, t996: F, t2902: F, t632: F) -> (F, F, F, F) {
    let t8515 = t466 * t8514;
    let t8517 = t8419 * t1001;
    let t8519 = 0.16221005325193686047e-3 * t8483 - 0.20855578275249024918e-2 * t8487 - 0.17714874716515957771e-4 * t8490 + 0.29524791194193262952e-5 * t8494 - 0.28840947468194373793e-3 * t8496 + 0.12360406057797588768e-3 * t8498 + 0.772525378612349298e-5 * t8502 - 0.45785004105758568397e-6 * t8506 - 0.27721444647547803303e-5 * t8512 + 0.6951859425083008306e-4 * t8515 - 0.26319242435966565832e-3 * t8517;
    let t8521 = t996 * t1266;
    let t8522 = t8521 * t1001;
    let t8524 = t2902 * t632;
    (t8519, t8521, t8522, t8524)
}
