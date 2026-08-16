//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1161/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1161(t2001: f64, t5574: f64, t13287: f64, t31195: f64, t39891: f64, t31259: f64, t31262: f64, t31277: f64, t31279: f64, t31284: f64, t31285: f64, t31291: f64, t31296: f64, t31297: f64, t31312: f64, t31316: f64, t31318: f64, t31322: f64, t35514: f64, t35515: f64, t37591: f64) -> f64 {
    let t40029 = t2001 * t5574;
    let t40034 = t31195 * t13287 * t39891;
    let t40040 = 0.196109375e0_f64 * t31259 + 0.13073958333333333333e0_f64 * t31262 - 0.19865625e0_f64 * t31277 - 0.1324375e0_f64 * t31279 + t35514 + 0.6431102717710460546e-2_f64 * t35515 - t31284 - 0.10289764348336736873e-1_f64 * t40029 - 0.53592522647587171215e-3_f64 * t31285 + t31291 - t37591 - t31296 - 0.15724046144802076034e-2_f64 * t31297 - 0.21437009059034868486e-2_f64 * t40034 - 0.42874018118069736972e-3_f64 * t31312 + 0.62896184579208304134e-3_f64 * t31316 + 0.56606566121287473723e-2_f64 * t31318 + 0.42874018118069736972e-3_f64 * t31322;
    t40040
}
