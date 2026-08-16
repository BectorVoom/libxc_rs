//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 757/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk757(t14022: f64, t14027: f64, t15339: f64, t458: f64, t1430: f64, t236: f64, t14121: f64, t14123: f64, t14125: f64, t21060: f64, t495: f64, t589: f64) -> (f64, f64, f64, f64) {
    let t73708 = t15339 * t458 * t14022 * t14027;
    let t73712 = t236 * t1430;
    let t73714 = t21060 * t14121 * t14123 * t14125 * t73712;
    let t73717 = t236 * t589 * t495;
    (t73708, t73712, t73714, t73717)
}
