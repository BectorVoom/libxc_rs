//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 833/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk833(t7335: f64, t8355: f64, t7345: f64, t1001: f64, t1475: f64, t1970: f64, t236: f64, t9210: f64, t35455: f64, t8451: f64, t7421: f64, t8571: f64) -> (f64, f64, f64, f64, f64) {
    let t38608 = t7335 * t8355;
    let t38610 = t7345 * t8355;
    let t38615 = t1970 * t9210 * t236 * t1475 * t1001;
    let t38617 = t8451 * t35455;
    let t38619 = t8571 * t7421;
    (t38608, t38610, t38615, t38617, t38619)
}
