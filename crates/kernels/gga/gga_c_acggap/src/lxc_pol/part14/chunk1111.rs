//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1111/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1111<F: Float>(t7433: F, t9637: F, t30638: F, t30655: F, t30658: F, t34557: F, t34562: F, t34571: F, t34582: F, t34586: F, t34593: F, t37147: F, t37148: F, t37163: F, t37166: F, t39382: F, t39386: F, t39389: F, t39391: F) -> F {
    let t39393 = t7433 * t9637;
    let t39399 = -t37147 - t37148 + t34557 + F::cast_from(0.15724046144802076034e-2_f64) * t39382 - F::cast_from(0.23586069217203114051e-2_f64) * t39386 + F::cast_from(0.31448092289604152068e-3_f64) * t39389 + t34562 + F::cast_from(0.13719685797782315831e-1_f64) * t39391 + F::cast_from(0.64311027177104605458e-3_f64) * t39393 - F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t30638 + t34571 - F::cast_from(0.21437009059034868486e-3_f64) * t30655 - t30658 + t37163 - F::cast_from(0.25158473831683321654e-2_f64) * t34582 + F::cast_from(0.37737710747524982481e-2_f64) * t34586 - t37166 - t34593;
    t39399
}
