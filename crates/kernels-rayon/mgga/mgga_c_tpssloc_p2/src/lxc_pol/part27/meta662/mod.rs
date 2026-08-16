//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta662 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2321;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2322;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2323;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2324;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta662(t16413: f64, t1985: f64, t1998: f64, t214: f64, t16248: f64, t22833: f64, t16383: f64, t16261: f64, t26309: f64, t22832: f64, t5234: f64, t3809: f64, t16405: f64, t16387: f64, t16275: f64, t16271: f64, t1336: f64, t22759: f64, t5252: f64, t836: f64, t26308: f64, t3777: f64, t16257: f64, t5293: f64, t80820: f64, t5259: f64, t80816: f64, t16244: f64, t5303: f64, t16366: f64, t16370: f64, t26257: f64, t3872: f64, t1831: f64, t80869: f64, t22783: f64, t5314: f64, t26297: f64, t80853: f64, t80855: f64, t26301: f64, t22788: f64, t16333: f64, t6952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91091, t91094, t91096, t91098, t91101) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2321(t16413, t1985, t1998, t214, t16248, t22833, t16383, t16261, t26309, t22832, t5234, t3809);
        let (t91103, t91105, t91107, t91109, t91114, t91116) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2322(t16405, t22833, t16387, t26309, t16275, t16271, t1336, t22759, t5252, t836, t26308, t3777);
        let t91132 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2323(t16257, t26309, t5293, t80820, t5259, t80816, t16244, t22833, t5303, t16366, t16370, t91094, t91096, t91098, t91101, t91103, t91105, t91107, t91109, t91114, t91116);
        let (t91133, t91136, t91138, t91141, t91144, t91145, t91147) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2324(t26257, t3872, t1831, t80869, t22783, t5314, t26297, t80853, t80855, t26301, t22788, t16333, t6952);
    (t91091, t91132, t91133, t91136, t91138, t91141, t91144, t91145, t91147)
}
