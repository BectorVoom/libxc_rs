//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1107/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1107<F: Float>(t14639: F, t79: F, t2736: F, t9512: F, t9532: F, t32185: F, t2744: F, t4534: F, t1610: F, t9555: F, t2705: F, t3443: F, t3460: F, t2701: F, t3441: F, t2692: F, t3274: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32473 = t14639 * t79;
    let t32474 = t32473 * t2736;
    let t32477 = t9512 * t9532;
    let t32502 = 0.38691203703703703703e-3 * t32185;
    let t32523 = t2744 * t4534;
    let t32533 = t9555 * t1610;
    let t32543 = t2705 * t3443;
    let t32546 = t2705 * t3460;
    let t32549 = t2701 * t3441;
    let t32552 = t2692 * t3274;
    (t32473, t32474, t32477, t32502, t32523, t32533, t32543, t32546, t32549, t32552)
}
