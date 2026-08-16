//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 756/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk756(t14121: f64, t14123: f64, t14125: f64, t21060: f64, t73712: f64, t236: f64, t495: f64, t589: f64, t69009: f64, t498: f64, t68421: f64, t68422: f64) -> (f64, f64, f64, f64, f64) {
    let t73714 = t21060 * t14121 * t14123 * t14125 * t73712;
    let t73717 = t236 * t589 * t495;
    let t73719 = t69009 * t14125 * t73717;
    let t73722 = t236 * t589 * t498;
    let t73724 = t68421 * t68422 * t73722;
    (t73714, t73717, t73719, t73722, t73724)
}
