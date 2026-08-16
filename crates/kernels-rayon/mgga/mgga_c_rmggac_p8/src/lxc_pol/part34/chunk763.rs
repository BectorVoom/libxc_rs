//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 763/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk763(t14015: f64, t73794: f64, t3154: f64, t38355: f64, t13858: f64, t8571: f64, t15363: f64, t69568: f64, t14236: f64, t14237: f64, t1528: f64, t2067: f64, t26: f64) -> (f64, f64, f64, f64, f64) {
    let t73801 = t73794 * t14015;
    let t73803 = t38355 * t3154;
    let t73805 = t8571 * t13858;
    let t73807 = t69568 * t15363;
    let t73812 = t14236 * t14237 * t2067 * t26 * t1528;
    (t73801, t73803, t73805, t73807, t73812)
}
