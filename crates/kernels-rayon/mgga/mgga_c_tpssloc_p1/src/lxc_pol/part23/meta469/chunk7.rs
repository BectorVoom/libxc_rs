//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1393/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1393(t76768: f64, t77498: f64, t77539: f64, t77587: f64, t77637: f64, t77687: f64, t77724: f64, t77761: f64, t1625: f64, t21390: f64, t5872: f64, t6739: f64) -> (f64, f64, f64) {
    let t77764 = t76768 + t77498 + t77539 + t77587 + t77637 + t77687 + t77724 + t77761;
    let t77782 = t1625 * t21390;
    let t77794 = t6739 * t5872;
    (t77764, t77782, t77794)
}
