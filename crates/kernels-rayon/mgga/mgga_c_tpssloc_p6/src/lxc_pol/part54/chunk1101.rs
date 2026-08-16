//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1101/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1101(t2148: f64, t4930: f64, t1716: f64, t7381: f64, t3502: f64, t491: f64, t24813: f64, t1011: f64, t1734: f64, t4978: f64, t1209: f64, t1216: f64) -> (f64, f64, f64, f64, f64) {
    let t27481 = t4930 * t2148;
    let t27484 = t1716 * t7381;
    let t27488 = t3502 * t491;
    let t27489 = t24813 * t27488;
    let t27490 = t1734 * t1011;
    let t27491 = t27490 * t4978;
    let t27492 = t27489 * t27491;
    let t27495 = t1209 * t491;
    let t27496 = t24813 * t27495;
    let t27497 = t27490 * t1216;
    (t27481, t27484, t27492, t27496, t27497)
}
