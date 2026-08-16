//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 876/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk876(t2145: f64, t27: f64, t5249: f64, t649: f64, t39157: f64, t39162: f64, t39167: f64, t39172: f64, t39177: f64, t39181: f64, t39184: f64, t39189: f64, t39193: f64, t39197: f64, t39200: f64, t39205: f64, t39209: f64, t39215: f64, t39219: f64, t39224: f64) -> f64 {
    let t39228 = t2145 * t27 * t649 * t5249;
    let t39230 = -0.51077519871957407276e-4_f64 * t39157 + 0.76616279807936110914e-4_f64 * t39162 + 0.25538759935978703638e-4_f64 * t39167 - 0.25538759935978703638e-4_f64 * t39172 + 0.31923449919973379548e-4_f64 * t39177 + 0.76616279807936110914e-4_f64 * t39181 - 0.76616279807936110914e-4_f64 * t39184 + 0.31923449919973379548e-4_f64 * t39189 - 0.15323255961587222183e-3_f64 * t39193 - 0.51077519871957407276e-4_f64 * t39197 + 0.51077519871957407276e-4_f64 * t39200 + 0.95770349759920138643e-4_f64 * t39205 + 0.1064114997332445985e-4_f64 * t39209 - 0.12769379967989351819e-4_f64 * t39215 - 0.42564599893297839398e-5_f64 * t39219 - 0.212822999466489197e-4_f64 * t39224 - 0.34093327067806677161e-2_f64 * t39228;
    t39230
}
