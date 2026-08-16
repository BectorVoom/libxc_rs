//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 804/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk804(t12200: f64, t1614: f64, t262: f64, t3068: f64, t41015: f64, t739: f64, t7577: f64, t14125: f64, t68440: f64, t9205: f64, t14224: f64, t8576: f64) -> (f64, f64, f64, f64) {
    let t74487 = t12200 * t3068 * t262 * t1614;
    let t74491 = 0.5987120850931904282e-1_f64 * t739 * t7577 * t41015;
    let t74495 = t68440 * t14125 * t9205;
    let t74497 = t8576 * t14224;
    (t74487, t74491, t74495, t74497)
}
