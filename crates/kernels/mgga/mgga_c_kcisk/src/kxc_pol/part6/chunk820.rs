//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 820/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk820<F: Float>(t645: F, t29188: F, t29259: F, t67: F, t2441: F, t8780: F, t11153: F, t1755: F, t28314: F, t2436: F, t2442: F, t28800: F, t340: F, t6141: F, t639: F, t642: F, t7196: F, t8773: F, t8781: F, t8787: F) -> (F, F, F, F) {
    let t646 = t645 < -0.66725e-1;
    let t29261 = t67 * (t29188 + t29259);
    let t29274 = t8780 * t2441;
    let t29275 = t11153 * t29274;
    let t29282 = t1755 * t28314;
    let t29287 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t29261 * t642 - 10.0 / 9.0 * t340 * t8773 * t2442 + 40.0 / 27.0 * t340 * t2436 * t8781 - 10.0 / 9.0 * t340 * t2436 * t8787 - 280.0 / 243.0 * t340 * t639 * t29275 + 40.0 / 27.0 * t6141 * t7196 * t28800 - 10.0 / 27.0 * t340 * t639 * t29282);
    (t29274, t29275, t29282, t29287)
}
