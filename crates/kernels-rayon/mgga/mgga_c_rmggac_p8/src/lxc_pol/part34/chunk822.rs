//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 822/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk822(t36634: f64, t656: f64, t8950: f64, t34944: f64, t8979: f64, t14125: f64, t68440: f64, t8835: f64, t8842: f64, t13962: f64, t3056: f64, t8486: f64) -> (f64, f64, f64, f64, f64) {
    let t74765 = t36634 * t656 * t8950;
    let t74768 = t34944 * t656 * t8979;
    let t74772 = t68440 * t14125 * t8835;
    let t74775 = t68440 * t14125 * t8842;
    let t74779 = t3056 * t13962 * t8486;
    (t74765, t74768, t74772, t74775, t74779)
}
