//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta819 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2883;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta819(t136: f64, t2826: f64, t59668: f64, t59672: f64, t10304: f64, t59725: f64, t59755: f64, t59746: f64, t908: f64, t4370: f64, t896: f64, t13634: f64, t13637: f64, t41959: f64, t41962: f64, t59680: f64, t59684: f64, t59688: f64, t59692: f64, t59694: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60223, t60226, t60229, t60232, t60235, t60237, t60238) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2883(t136, t2826, t59668, t59672, t10304, t59725, t59755, t59746, t908, t4370, t896, t13634);
        let (t60240, t60242) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2884(t13637, t60237, t41959, t41962, t59680, t59684, t59688, t59692, t59694, t60223, t60226, t60229, t60232, t60235, t60238);
    (t60223, t60226, t60229, t60232, t60235, t60238, t60240, t60242)
}
