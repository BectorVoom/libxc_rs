//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 950/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk950(t3928: f64, t5199: f64, t645: f64, t118: f64, t1986: f64, t352: f64, t39866: f64, t7717: f64, t1971: f64, t2144: f64, t7230: f64, t8834: f64) -> (f64, f64, f64) {
    let t40307 = t3928 * t645 * t5199;
    let t40313 = t1986 * t118 * t39866 * t352;
    let t40314 = t7717 * t40313;
    let t40319 = t7230 * t1971 * t2144 * t8834 * t352;
    (t40307, t40314, t40319)
}
