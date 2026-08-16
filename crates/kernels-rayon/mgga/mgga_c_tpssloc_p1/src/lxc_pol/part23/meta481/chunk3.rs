//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1442/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1442(t1740: f64, t48: f64, t338: f64, t11546: f64, t1174: f64, t15390: f64, t18321: f64, t3447: f64, t44566: f64, t463: f64, t4919: f64, t52124: f64, t6127: f64, t64878: f64, t64881: f64, t64885: f64, t64979: f64, t73433: f64, t73444: f64, t73451: f64, t75836: f64, sigma2: f64) -> (f64, f64, f64) {
    let t78504 = 1.0_f64 / t48 / t1740;
    let t78505 = sigma2 * t78504;
    let t78506 = t78505 * t338;
    let t78516 = -0.32592592592592592592e-1_f64 * t73433 - 0.32921810699588477364e-2_f64 * t52124 + 0.66666666666666666664e-2_f64 * t3447 * t4919 * t73444 - 0.44444444444444444444e-2_f64 * t3447 * t15390 * t73451 - 0.1086419753086419753e-1_f64 * t64878 + 0.11111111111111111111e-2_f64 * t64881 + 0.11111111111111111111e-2_f64 * t64885 + 0.21547325102880658436e0_f64 * t78506 * t463 - 0.1037037037037037037e-1_f64 * t1174 * t11546 * t44566 * t75836 - 0.32592592592592592591e-1_f64 * t18321 * t6127 + 0.37037037037037037036e-3_f64 * t64979;
    (t78505, t78506, t78516)
}
