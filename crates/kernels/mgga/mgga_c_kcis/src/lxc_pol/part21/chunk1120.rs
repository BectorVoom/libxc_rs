//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1120/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1120<F: Float>(t14629: F, t95463: F, t3178: F, t5068: F, t389: F, t42385: F, t5096: F, t14739: F, t26930: F, t1169: F, t376: F, t14650: F, t95453: F, t95455: F, t95457: F, t95459: F, t95461: F) -> (F, F, F, F, F, F, F) {
    let t95464 = t95463 * t14629;
    let t95466 = t3178 * t5068;
    let t95468 = t42385 * t389;
    let t95470 = t3178 * t5096;
    let t95472 = t26930 * t14739;
    let t95474 = t1169 * t376;
    let t95475 = t95474 * t14650;
    let t95477 = 11.0 / 27.0 * t95453 - 3.0 / 8.0 * t95455 + t95457 / 288.0 - t95459 / 24.0 - t95461 / 12.0 + t95464 / 6.0 + t95466 / 64.0 - t95468 / 128.0 - t95470 / 12.0 + t95472 / 48.0 + t95475 / 36.0;
    (t95464, t95466, t95468, t95470, t95472, t95475, t95477)
}
