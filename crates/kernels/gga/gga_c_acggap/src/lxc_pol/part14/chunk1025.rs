//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1025/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1025<F: Float>(t2001: F, t5574: F, t13287: F, t31195: F, t39891: F, t31259: F, t31262: F, t31277: F, t31279: F, t31284: F, t31285: F, t31291: F, t31296: F, t31297: F, t31312: F, t31316: F, t31318: F, t31322: F, t35514: F, t35515: F, t37591: F) -> (F,) {
    let t40029 = t2001 * t5574;
    let t40034 = t31195 * t13287 * t39891;
    let t40040 = 0.196109375e0 * t31259 + 0.13073958333333333333e0 * t31262 - 0.19865625e0 * t31277 - 0.1324375e0 * t31279 + t35514 + 0.6431102717710460546e-2 * t35515 - t31284 - 0.10289764348336736873e-1 * t40029 - 0.53592522647587171215e-3 * t31285 + t31291 - t37591 - t31296 - 0.15724046144802076034e-2 * t31297 - 0.21437009059034868486e-2 * t40034 - 0.42874018118069736972e-3 * t31312 + 0.62896184579208304134e-3 * t31316 + 0.56606566121287473723e-2 * t31318 + 0.42874018118069736972e-3 * t31322;
    (t40040,)
}
