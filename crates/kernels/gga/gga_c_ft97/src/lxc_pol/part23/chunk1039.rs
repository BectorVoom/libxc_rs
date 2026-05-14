//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1039/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1039<F: Float>(t10570: F, t31572: F, t1486: F, t193: F, t1476: F, t5337: F, t852: F, t25027: F, t5362: F, t6308: F, t28491: F, t28494: F, t28529: F, t28531: F, t28784: F, t31364: F, t31368: F, t31372: F, t31376: F, t31554: F, t31562: F, t31566: F, t31570: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31573 = t10570 * t31572;
    let t31575 = t1486 * t193 * t31573;
    let t31577 = t1476 * t5337;
    let t31578 = t852 * t31577;
    let t31580 = t25027 * t193 * t31578;
    let t31582 = t1476 * t5362;
    let t31583 = t852 * t31582;
    let t31585 = t6308 * t193 * t31583;
    let t31587 = -2.0 / 3.0 * t31364 + t31368 / 3.0 + 2.0 / 9.0 * t31372 + 2.0 / 3.0 * t31376 - t31554 / 2.0 + 2.0 / 3.0 * t28491 - t28494 / 6.0 + t28529 / 3.0 - 2.0 / 9.0 * t28531 - t31562 / 6.0 - 2.0 / 3.0 * t31566 - t28784 / 9.0 - 4.0 / 3.0 * t31570 - 3.0 * t31575 - 3.0 / 8.0 * t31580 + t31585 / 4.0;
    (t31573, t31575, t31577, t31578, t31580, t31582, t31583, t31585, t31587)
}
