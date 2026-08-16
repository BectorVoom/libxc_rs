//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta628 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2072;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta628(t1235: f64, t225: f64, t461: f64, t24574: f64, t24626: f64, t24617: f64, t11553: f64, t2121: f64, t2123: f64, t2122: f64, t85628: f64, t24884: f64, t7288: f64, t85660: f64, t24758: f64, t24637: f64, t7294: f64, t3427: f64, t7295: f64, t24901: f64, t3640: f64, t11947: f64, t7394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t86415, t86424, t86426, t86451, t86452, t86456) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2072(t1235, t225, t461, t24574, t24626, t24617, t11553, t2121, t2123, t2122, t85628, t24884);
        let (t86473, t86475, t86494, t86501, t86513, t86517) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2073(t7288, t85660, t225, t24758, t24637, t7294, t2121, t3427, t7295, t24901, t3640, t11947, t7394);
    (t86415, t86424, t86426, t86451, t86452, t86456, t86473, t86475, t86494, t86501, t86513, t86517)
}
