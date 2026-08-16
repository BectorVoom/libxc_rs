//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1010/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1010(t35: f64, t85424: f64, t11160: f64, t11232: f64, t11233: f64, t15680: f64, t15782: f64, t15789: f64, t15793: f64, t15797: f64, t1594: f64, t1631: f64, t20007: f64, t372: f64, t38200: f64, t38211: f64, t58407: f64, t73906: f64, t74009: f64, t74034: f64, t85414: f64) -> f64 {
    let t85574 = t85424 * t35;
    let t85602 = 0.58097170218823199823e-3_f64 * t372 * t1594 * t85574 + 0.20914981278776351936e-3_f64 * t372 * t38211 * t85414 - 0.40559281352147498558e-3_f64 * t15797 * t58407 - 0.24335568811288499135e-3_f64 * t15789 * t15782 - 0.279058811357253504e-1_f64 * t11232 * t11233 * t20007 + 0.27039520901431665705e-3_f64 * t15793 * t74009 + 0.69764702839313376e-2_f64 * t372 * t1631 * t85574 + 0.53719526674014200183e-7_f64 * t372 * t38200 * t85414 + 0.16223712540858999423e-3_f64 * t73906 * t11160 - 0.20265659080606036993e-4_f64 * t15680 * t15782 + 0.13510439387070691329e-4_f64 * t74034 * t11160;
    t85602
}
