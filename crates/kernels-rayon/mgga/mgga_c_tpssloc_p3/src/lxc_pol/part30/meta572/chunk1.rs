//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1943/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1943(t5677: f64, t6785: f64, t23696: f64, t1945: f64, t5866: f64, t1060: f64, t25470: f64, t7603: f64, t1409: f64, t1615: f64, t6800: f64, t23635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28637 = t6785 * t5677;
    let t28638 = t23696 * t28637;
    let t28641 = t1945 * t5866;
    let t28642 = t28641 * t1060;
    let t28648 = t25470 * t7603;
    let t28651 = t1409 * t1615;
    let t28652 = t28651 * t6800;
    let t28653 = t23635 * t28652;
    (t28637, t28638, t28641, t28642, t28648, t28651, t28652, t28653)
}
