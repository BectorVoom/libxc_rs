//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 952/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk952(t74195: f64, t68525: f64, t70877: f64, t74166: f64, t74168: f64, t74183: f64, t74197: f64, t76884: f64, t76885: f64, t76886: f64, t76887: f64, t76888: f64, t76892: f64, t76893: f64, t76894: f64, t76896: f64, t76897: f64) -> f64 {
    let t76898 = 0.25650144397517585626e-6_f64 * t74195;
    let t76900 = -0.29085809927086856923e-4_f64 * t74166 + 0.29085809927086856923e-4_f64 * t74168 + t76884 - t76885 + t76886 + t76887 - t76888 - 0.72714524817717142308e-5_f64 * t74183 + t76892 - t76893 - t76894 + t70877 + 0.29085809927086856923e-4_f64 * t68525 - t76896 - t76897 - t76898 - 0.57000320883372412496e-7_f64 * t74197;
    t76900
}
