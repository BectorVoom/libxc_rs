//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1111/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1111(t7433: f64, t9637: f64, t30638: f64, t30655: f64, t30658: f64, t34557: f64, t34562: f64, t34571: f64, t34582: f64, t34586: f64, t34593: f64, t37147: f64, t37148: f64, t37163: f64, t37166: f64, t39382: f64, t39386: f64, t39389: f64, t39391: f64) -> f64 {
    let t39393 = t7433 * t9637;
    let t39399 = -t37147 - t37148 + t34557 + 0.15724046144802076034e-2_f64 * t39382 - 0.23586069217203114051e-2_f64 * t39386 + 0.31448092289604152068e-3_f64 * t39389 + t34562 + 0.13719685797782315831e-1_f64 * t39391 + 0.64311027177104605458e-3_f64 * t39393 - 35.0_f64 / 432.0_f64 * t30638 + t34571 - 0.21437009059034868486e-3_f64 * t30655 - t30658 + t37163 - 0.25158473831683321654e-2_f64 * t34582 + 0.37737710747524982481e-2_f64 * t34586 - t37166 - t34593;
    t39399
}
