//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta652 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta652(t7611: f64, t82716: f64, t25550: f64, t82822: f64, t23384: f64, t25476: f64, t25467: f64, t25459: f64, t7604: f64, t82632: f64, t25723: f64, t88810: f64, t1539: f64, t6746: f64, t82655: f64, t14220: f64, t7581: f64, t25555: f64, t25529: f64, t6680: f64, t1920: f64, t2966: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t89310, t89327, t89329, t89360, t89362, t89366, t89369) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066(t7611, t82716, t25550, t82822, t23384, t25476, t25467, t25459, t7604, t82632, t25723, t88810);
        let (t89395, t89399, t89421, t89429, t89431) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2067(t1539, t6746, t82655, t14220, t7581, t25555, t82822, t25529, t6680, t1920, t2966, t7614);
    (t89310, t89327, t89329, t89360, t89362, t89366, t89369, t89395, t89399, t89421, t89429, t89431)
}
