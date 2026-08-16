//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta584(t19735: f64, t5335: f64, t1824: f64, t1834: f64, t5250: f64, t562: f64, t6387: f64, t12250: f64, t1351: f64, t5287: f64, t5348: f64, t1336: f64, t16047: f64, t19654: f64, t19658: f64, t19661: f64, t19668: f64, t19674: f64, t19733: f64, t3777: f64, t5234: f64, t5334: f64, t5336: f64, t5349: f64, t6448: f64, t6451: f64, t6454: f64, t6456: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19736, t19739) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2314(t19735, t5335, t1824, t1834);
        let (t19740, t19743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2315(t19739, t5250, t562, t6387);
        let (t19744, t19745, t19748, t19752, t19755) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2316(t12250, t1351, t19743, t5250, t5287, t5348, t1336, t16047, t19654, t19658, t19661, t19668, t19674, t19733, t19736, t19740, t3777, t5234, t5334, t5336, t5349, t6448, t6451, t6454, t6456);
    (t19736, t19739, t19740, t19743, t19744, t19745, t19748, t19752, t19755)
}
