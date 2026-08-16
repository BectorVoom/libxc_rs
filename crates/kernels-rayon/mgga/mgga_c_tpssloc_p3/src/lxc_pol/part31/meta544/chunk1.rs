//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1768/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1768(t6612: f64, t812: f64, t836: f64, t2690: f64, t6619: f64, t849: f64, t23132: f64, t2617: f64, t131: f64, t23121: f64, t9537: f64, t236: f64, t81613: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81749 = t812 * t6612 * t836;
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    let t81769 = t2617 * t23132;
    let t81782 = t23121 * t131 * t9537;
    let t81783 = t81613 * t236;
    (t81749, t81763, t81764, t81769, t81782, t81783)
}
