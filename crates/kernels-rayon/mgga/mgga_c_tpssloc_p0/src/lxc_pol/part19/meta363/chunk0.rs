//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1323/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323(t1012: f64, t1015: f64, t1017: f64, t10444: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t41726: f64, t41728: f64, t41732: f64, t41737: f64) -> (f64, f64) {
    let t42658 = t1012 * t1015 * t10444 * t1017;
    let t42661 = t41620 + t41622 + t41625 + t41627 + t41635 + t41639 - t41722 - t41726 + t41728 + t41732 + t41737;
    (t42658, t42661)
}
