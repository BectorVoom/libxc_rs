//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1483/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1483(t44878: f64, t44943: f64, t44999: f64, t45066: f64, t45133: f64, t45186: f64, t45246: f64, t45311: f64, t3609: f64, t44927: f64, t3623: f64, t11880: f64, t44690: f64) -> (f64, f64, f64, f64) {
    let t45314 = t44878 + t44943 + t44999 + t45066 + t45133 + t45186 + t45246 + t45311;
    let t45320 = t44927 * t3609;
    let t45323 = t44927 * t3623;
    let t45326 = t44690 * t11880;
    (t45314, t45320, t45323, t45326)
}
