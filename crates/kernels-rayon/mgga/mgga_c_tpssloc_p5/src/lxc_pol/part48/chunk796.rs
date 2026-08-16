//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 796/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk796(t1011: f64, t3507: f64, t3508: f64, t24661: f64, t1209: f64, t3030: f64, t478: f64, t475: f64, t1222: f64, t7334: f64, t2140: f64, t3566: f64) -> (f64, f64, f64, f64) {
    let t24662 = t3507 * t1011;
    let t24663 = t24662 * t3508;
    let t24664 = t24661 * t24663;
    let t24667 = t3030 * t1209;
    let t24668 = t24667 * t478;
    let t24669 = t24662 * t475;
    let t24670 = t24668 * t24669;
    let t24675 = t7334 * t1222;
    let t24677 = t3566 * t2140;
    (t24664, t24670, t24675, t24677)
}
