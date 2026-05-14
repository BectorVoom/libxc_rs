//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 743/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk743<F: Float>(t1441: F, t9469: F, t415: F, t1299: F, t467: F, t470: F, t454: F, t1451: F, t468: F, t2718: F, t9422: F, t9426: F, t9429: F, t9434: F, t9439: F, t9444: F, t9446: F, t9449: F, t9454: F, t9460: F, t9464: F, t9467: F) -> (F, F, F, F, F, F, F, F) {
    let t9470 = t9469 * t1441;
    let t9471 = t415 * t9470;
    let t9474 = t467 * t1299 * t470;
    let t9475 = t454 * t9474;
    let t9476 = t415 * t9475;
    let t9478 = t468 * t1451;
    let t9479 = t415 * t9478;
    let t9481 = -0.10416666666666666667e-1 * t9422 * t2718 + 0.40208333333333333335e-2 * t9426 * t9429 - 0.10416666666666666667e-1 * t9434 * t2718 + 0.27777777777777777779e-1 * t9439 * t2718 - t9444 - 0.34722222222222222223e-2 * t9446 * t9449 + 0.10416666666666666667e-1 * t9446 * t9454 + 0.10416666666666666667e-1 * t9446 * t9429 + t9460 + 0.16581944444444444444e-2 * t9464 + 0.24872916666666666666e-2 * t9467 - 0.24872916666666666666e-2 * t9471 - 0.66327777777777777776e-2 * t9476 + 0.16581944444444444444e-2 * t9479;
    (t9470, t9471, t9474, t9475, t9476, t9478, t9479, t9481)
}
