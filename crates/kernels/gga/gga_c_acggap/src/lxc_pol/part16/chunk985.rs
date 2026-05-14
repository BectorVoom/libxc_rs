//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 985/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk985<F: Float>(t1181: F, t604: F, t6203: F, t7575: F, t1165: F, t6209: F, t7351: F, t20417: F, t2068: F, t2073: F, t31346: F, t5932: F, t7433: F, t9637: F, t30638: F, t30655: F, t30658: F, t34557: F, t34562: F, t34571: F, t34582: F, t34586: F, t34593: F, t37147: F, t37148: F, t37163: F, t37166: F) -> (F,) {
    let t39382 = t7575 * t1181 * t604 * t6203;
    let t39386 = t7575 * t1165 * t7351 * t6209;
    let t39389 = t2068 * t20417 * t2073;
    let t39391 = t31346 * t5932;
    let t39393 = t7433 * t9637;
    let t39399 = -t37147 - t37148 + t34557 + 0.15724046144802076034e-2 * t39382 - 0.23586069217203114051e-2 * t39386 + 0.31448092289604152068e-3 * t39389 + t34562 + 0.13719685797782315831e-1 * t39391 + 0.64311027177104605458e-3 * t39393 - 35.0 / 432.0 * t30638 + t34571 - 0.21437009059034868486e-3 * t30655 - t30658 + t37163 - 0.25158473831683321654e-2 * t34582 + 0.37737710747524982481e-2 * t34586 - t37166 - t34593;
    (t39399,)
}
