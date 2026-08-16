//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2226;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2227;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2228;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta667<F: Float>(t16413: F, t1985: F, t1998: F, t214: F, t16248: F, t22833: F, t16383: F, t16261: F, t26309: F, t22832: F, t5234: F, t3809: F, t16405: F, t16387: F, t16275: F, t16271: F, t1336: F, t22759: F, t5252: F, t836: F, t26308: F, t3777: F, t16257: F, t5293: F, t80820: F, t5259: F, t80816: F, t16244: F, t5303: F, t16366: F, t16370: F, t26257: F, t3872: F, t1831: F, t80869: F, t22783: F, t5314: F, t26297: F, t80853: F, t80855: F, t26301: F, t22788: F, t16333: F, t6952: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t91091, t91094, t91096, t91098, t91101) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2226::<F>(t16413, t1985, t1998, t214, t16248, t22833, t16383, t16261, t26309, t22832, t5234, t3809);
        let (t91103, t91105, t91107, t91109, t91114, t91116) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2227::<F>(t16405, t22833, t16387, t26309, t16275, t16271, t1336, t22759, t5252, t836, t26308, t3777);
        let t91132 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2228::<F>(t16257, t26309, t5293, t80820, t5259, t80816, t16244, t22833, t5303, t16366, t16370, t91094, t91096, t91098, t91101, t91103, t91105, t91107, t91109, t91114, t91116);
        let (t91133, t91136, t91138, t91141, t91144, t91145, t91147) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2229::<F>(t26257, t3872, t1831, t80869, t22783, t5314, t26297, t80853, t80855, t26301, t22788, t16333, t6952);
    (t91091, t91132, t91133, t91136, t91138, t91141, t91144, t91145, t91147)
}
