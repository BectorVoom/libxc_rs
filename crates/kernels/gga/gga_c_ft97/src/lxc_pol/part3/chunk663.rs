//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 663/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk663<F: Float>(t383: F, t77: F, t3020: F, t384: F, t938: F, t4449: F, t6: F, t1620: F, t4467: F, t408: F, t4491: F, t11247: F, t11361: F, t15674: F, t15677: F, t15680: F, t1617: F, t1624: F, t1669: F, t3076: F, t3077: F, t3099: F, t372: F, t399: F, t401: F, t4476: F, t4493: F, t8009: F, t8015: F) -> (F, F, F) {
    let t15681 = t77 * t383;
    let t15682 = t3020 * t15681;
    let t15689 = t384 * t938;
    let t15693 = t4449 * t6;
    let t15694 = t15693 * t1620;
    let t15697 = t4467 * t6;
    let t15706 = t408 * t4491;
    let t15710 = 0.11627450473218896e-1 * t1624 * t15674 + 0.23254900946437792e-2 * t372 * t15677 - 0.33776098467676728323e-5 * t15680 * t15682 - 0.59273806478425129876e-2 * t4493 * t399 + 0.11854761295685025975e-1 * t4476 * t399 - 0.38731446812548799881e-3 * t11361 * t11247 * t15689 + 0.13784064983740990796e-4 * t8015 * t15694 - 0.68920324918704953981e-4 * t1617 * t15697 * t1620 + 0.45915205659928668026e-5 * t8009 * t15694 + 4.0 * t3076 * t3077 * t3099 - 2.0 * t1669 * t15706 * t401;
    (t15682, t15689, t15710)
}
