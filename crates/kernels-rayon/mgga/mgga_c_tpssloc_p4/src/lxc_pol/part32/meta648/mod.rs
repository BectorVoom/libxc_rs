//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta648 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2071;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2072;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta648(t90961: f64, t26415: f64, t81159: f64, t26418: f64, t6914: f64, t7736: f64, t80854: f64, t81064: f64, t22704: f64, t22705: f64, t26410: f64, t26432: f64, t6897: f64, t794: f64, t22642: f64, t22690: f64, t26395: f64, t22863: f64, t7737: f64, t26448: f64, t90497: f64, t215: f64, t6916: f64, t225: f64, t3787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90962, t90964, t90971, t90980, t90984, t90987) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2071(t90961, t26415, t81159, t26418, t6914, t7736, t80854, t81064, t22704, t22705, t26410, t26432, t6897, t794);
        let (t90988, t90993, t91000, t91002, t91004, t91005) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2072(t90987, t22642, t22690, t26395, t22863, t7737, t26448, t90497, t215, t6916, t225, t3787);
    (t90962, t90964, t90971, t90980, t90984, t90988, t90993, t91000, t91002, t91004, t91005)
}
