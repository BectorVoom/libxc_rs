//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1204/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1204(t12737: f64, t12741: f64, t12743: f64, t12746: f64, t12751: f64, t12753: f64, t12758: f64, t12760: f64, t12763: f64, t12766: f64, t12768: f64, t12771: f64, t12775: f64, t12778: f64, t12783: f64, t12785: f64, t12788: f64, t12794: f64, t12798: f64, t12801: f64, t12803: f64, t12808: f64, t12810: f64) -> (f64, f64) {
    let t14388 = -t12737 - t12741 - t12743 - t12746 - t12751 + t12753 + t12758 - t12760 - t12763 - t12766 - t12768;
    let t14389 = -t12771 - t12775 - t12778 - t12783 - t12785 - t12788 + t12794 + t12798 - t12801 + t12803 - t12808 + t12810;
    (t14388, t14389)
}
