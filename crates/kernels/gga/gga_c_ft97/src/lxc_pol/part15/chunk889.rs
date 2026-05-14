//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 889/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk889<F: Float>(t35: F, t85424: F, t11160: F, t11232: F, t11233: F, t15680: F, t15782: F, t15789: F, t15793: F, t15797: F, t1594: F, t1631: F, t20007: F, t372: F, t38200: F, t38211: F, t58407: F, t73906: F, t74009: F, t74034: F, t85414: F) -> (F,) {
    let t85574 = t85424 * t35;
    let t85602 = 0.58097170218823199823e-3 * t372 * t1594 * t85574 + 0.20914981278776351936e-3 * t372 * t38211 * t85414 - 0.40559281352147498558e-3 * t15797 * t58407 - 0.24335568811288499135e-3 * t15789 * t15782 - 0.279058811357253504e-1 * t11232 * t11233 * t20007 + 0.27039520901431665705e-3 * t15793 * t74009 + 0.69764702839313376e-2 * t372 * t1631 * t85574 + 0.53719526674014200183e-7 * t372 * t38200 * t85414 + 0.16223712540858999423e-3 * t73906 * t11160 - 0.20265659080606036993e-4 * t15680 * t15782 + 0.13510439387070691329e-4 * t74034 * t11160;
    (t85602,)
}
