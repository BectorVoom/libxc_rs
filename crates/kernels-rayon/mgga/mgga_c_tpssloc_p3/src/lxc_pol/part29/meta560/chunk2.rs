//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1965/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1965(t27602: f64, t27648: f64, t27679: f64, t27719: f64, t493: f64, t1734: f64, t7348: f64, t1246: f64, t24574: f64, t8070: f64, t2147: f64, t5052: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27721 = t27602 + t27648 + t27679 + t27719;
    let t27722 = t493 * t27721;
    let t27724 = t7348 * t1734;
    let t27725 = t27724 * t1246;
    let t27728 = t24574 * t8070;
    let t27732 = t2147 * t5052;
    (t27721, t27722, t27724, t27725, t27728, t27732)
}
