//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 959/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk959<F: Float>(t17630: F, t4431: F, t1073: F, t12116: F, t12122: F, t20027: F, t20035: F, t2265: F, t2266: F, t4462: F, t48117: F, t4883: F, t75994: F, t76056: F, t76062: F, t76101: F, t76126: F, t76128: F, t76130: F, t8654: F) -> (F,) {
    let t87843 = t17630 * t4431;
    let t87868 = -8.0 * t75994 - 160.0 / 81.0 * t48117 + 8.0 * t2265 * t12116 * t87843 - 4.0 / 3.0 * t2265 * t12122 * t87843 - 16.0 / 3.0 * t76056 + 8.0 / 3.0 * t76062 - 4.0 / 9.0 * t76101 + 8.0 / 9.0 * t76126 + 8.0 / 3.0 * t76128 + 8.0 / 3.0 * t76130 + 8.0 / 3.0 * t2265 * t8654 * t20027 * t1073 - 2.0 * t2265 * t2266 * t4462 * t4883 - 8.0 * t2265 * t2266 * t20035 * t1073;
    (t87868,)
}
