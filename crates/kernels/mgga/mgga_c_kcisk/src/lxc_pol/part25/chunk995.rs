//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 995/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk995<F: Float>(t17423: F, t17446: F, t17458: F, t17463: F, t17469: F, t17472: F, t17475: F, t17478: F, t17482: F, t17485: F, t17488: F, t17492: F, t10988: F, t10997: F, t11001: F, t11013: F, t17375: F, t17379: F, t17382: F, t17385: F, t17388: F, t17391: F, t17417: F, t17420: F, t17426: F, t17429: F, t17432: F, t17435: F, t17437: F, t17440: F, t17442: F, t17598: F) -> (F,) {
    let t17602 = 0.44152e0 * t17423;
    let t17622 = 0.16504875e0 * t17446 + 0.258925e1 * t17469 - 0.99342e0 * t17472 - 0.5519e-1 * t17475 - 0.73586666666666666666e-1 * t17478 + 0.22076e0 * t17482 + 0.33114e0 * t17485 - 0.132456e1 * t17488 + 0.12077e1 * t17458 - 0.181155e1 * t17463 + 0.16504875e0 * t17492;
    let t17624 = -0.22076e0 * t10988 + 0.11038e0 * t10997 + 0.36793333333333333333e-1 * t11001 - 0.36793333333333333334e0 * t11013 - 0.143494e1 * t17375 - 0.22141166666666666666e1 * t17379 - 0.13418888888888888889e0 * t17382 - 0.18396666666666666667e0 * t17385 + 0.19419375e1 * t17388 - 0.412621875e-1 * t17391 + t17598 + 0.60385e0 * t17417 - 0.24154e1 * t17420 - t17602 + 0.73586666666666666666e-1 * t17426 + 0.33114e0 * t17429 + 0.33114e0 * t17432 + 0.60385e0 * t17435 + 0.82524375e-1 * t17437 - 0.258925e1 * t17440 - 0.1294625e1 * t17442 + t17622;
    (t17624,)
}
