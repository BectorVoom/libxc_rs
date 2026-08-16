//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 468/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk468(t11674: f64, t321: f64, t333: f64, t352: f64, t1614: f64, t26: f64, t2564: f64, t11644: f64, t11648: f64, t117: f64, t5011: f64, t11662: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11675 = t11674 * t321;
    let t11679 = t11674 * t333;
    let t11683 = t11674 * t352;
    let t11703 = t26 * t1614;
    let t11704 = t2564 * t11703;
    let t11723 = t2564 * t11679;
    let t11729 = t2564 * t11644;
    let t11732 = t2564 * t11648;
    let t11905 = t5011 * t117;
    let t12012 = t2564 * t11683;
    let t12108 = t2564 * t11662;
    (t11675, t11679, t11683, t11703, t11704, t11723, t11729, t11732, t11905, t12012, t12108)
}
