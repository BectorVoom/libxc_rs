//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 872/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk872(t656: f64, t668: f64, t691: f64, t2617: f64, t2623: f64, t195: f64, t2838: f64, t2955: f64, t2614: f64, t2981: f64, t951: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12664 = 0.12842595503380418954e1_f64 * t656 * t668 * t691;
    let t12665 = t2617 * t2623;
    let t12669 = 0.38527786510141256862e1_f64 * t656 * t195 * t2838;
    let t12672 = 0.38025319932552508021e2_f64 * t656 * t195 * t2955;
    let t12673 = t2617 * t2614;
    let t12677 = 0.21687162600603479684e-1_f64 * t656 * t195 * t2981;
    let t12719 = t980 * t951;
    (t12664, t12665, t12669, t12672, t12673, t12677, t12719)
}
