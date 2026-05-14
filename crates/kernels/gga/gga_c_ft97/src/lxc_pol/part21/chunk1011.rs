//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1011/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1011<F: Float>(t1526: F, t3338: F, t7705: F, t38308: F, t4641: F, t3323: F, t2252: F, t342: F, t4645: F, t16654: F, t630: F, t1609: F, t51: F, t3018: F, t7906: F, t398: F, t4491: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t64655 = t1526 * t7705 * t3338 / 18.0;
    let t64663 = t1526 * t38308 * t4641;
    let t64668 = t1526 * t7705 * t3323 / 18.0;
    let t64677 = t342 * t2252 * t4645;
    let t64681 = t342 * t630 * t16654 / 6.0;
    let t65750 = t51 * t1609;
    let t73772 = t7906 * t3018;
    let t73784 = t4491 * t6 * t51 * t398;
    (t64655, t64663, t64668, t64677, t64681, t65750, t73772, t73784)
}
