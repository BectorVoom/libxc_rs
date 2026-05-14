//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 940/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk940<F: Float>(t2105: F, t27613: F, t2293: F, t8365: F, t2292: F, t27584: F, t2297: F, t7819: F, t8349: F, t4463: F, t19100: F, t25590: F, t25601: F, t25609: F, t25696: F, t25699: F, t25701: F, t30569: F, t30572: F, t30582: F, t30585: F, t30606: F, t30608: F, t30610: F) -> (F, F, F, F, F, F, F) {
    let t31509 = t27613 * t2105;
    let t31512 = t2293 * t8365;
    let t31515 = t27584 * t2292;
    let t31518 = t2297 * t7819;
    let t31525 = t8349 * t2292;
    let t31526 = t31525 * t4463;
    let t31543 = -0.103295e1 * t30569 + 0.309885e1 * t30572 - 0.68863333333333333332e0 * t19100 + 0.34431666666666666666e0 * t25590 - 0.103295e1 * t25601 + 0.51647499999999999999e0 * t25609 - 0.41678000000000000001e0 * t25696 + 0.20839e0 * t25699 + 0.69463333333333333335e-1 * t25701 - 0.104195e0 * t30582 + 0.62517e0 * t30585 + 0.6311625e0 * t30606 + 0.3529725e1 * t30608 - 0.52945875e1 * t30610;
    (t31509, t31512, t31515, t31518, t31525, t31526, t31543)
}
