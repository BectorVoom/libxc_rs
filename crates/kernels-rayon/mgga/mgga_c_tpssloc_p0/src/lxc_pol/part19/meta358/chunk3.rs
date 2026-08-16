//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1302/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1302(t300: f64, t41790: f64, t41993: f64, t42122: f64, t42270: f64, t1068: f64, t11087: f64, t3216: f64, t41620: f64, t41622: f64, t41625: f64, t41627: f64, t41635: f64, t41639: f64, t41722: f64, t41726: f64, t41728: f64, t41732: f64, t41737: f64, t4700: f64) -> (f64, f64) {
    let t42273 = t300 * (t41790 + t41993 + t42122 + t42270);
    let t42274 = -4.0_f64 * t1068 * t11087 * t3216 * t4700 + t41620 + t41622 + t41625 + t41627 + t41635 + t41639 - t41722 - t41726 + t41728 + t41732 + t41737 + t42273;
    (t42273, t42274)
}
