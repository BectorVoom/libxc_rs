//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 758/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk758(t14125: f64, t69009: f64, t73717: f64, t236: f64, t498: f64, t589: f64, t68421: f64, t68422: f64, t321: f64, t21714: f64, t333: f64, t511: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73719 = t69009 * t14125 * t73717;
    let t73722 = t236 * t589 * t498;
    let t73724 = t68421 * t68422 * t73722;
    let t73727 = t236 * t589 * t321;
    let t73729 = t68421 * t21714 * t73727;
    let t73732 = t511 * t589 * t333;
    (t73719, t73722, t73724, t73727, t73729, t73732)
}
