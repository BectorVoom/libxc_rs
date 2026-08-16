//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 483/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk483(t305: f64, t326: f64, t3814: f64, t3819: f64, t3826: f64, t3839: f64, t3851: f64, t4895: f64, t4928: f64, t5181: f64, t5194: f64, t5199: f64, t5204: f64, t5207: f64, t5211: f64, t5218: f64, t5223: f64, t5226: f64, t5245: f64, t551: f64, t797: f64, t851: f64, t854: f64, t861: f64) -> f64 {
    let t5248 = -0.11974241701863808564e0_f64 * t797 * t5211 + 0.26552308210121162678e-2_f64 * t851 * t5204 + 0.13276154105060581339e-2_f64 * t851 * t5207 - 0.59871208509319042821e-1_f64 * t797 * t5218 + 0.23948483403727617128e0_f64 * t3814 * t5181 - 0.148692925976678511e-1_f64 * t3819 * t5223 - 0.79656924630363488035e-2_f64 * t3826 * t5226 - 0.15931384926072697607e-2_f64 * t854 * t5218 + 0.3717323149416962775e-2_f64 * t861 * t5194 - 0.31862769852145395214e-2_f64 * t854 * t5211 - 0.15931384926072697607e-2_f64 * t854 * t5199 - 0.39914139006212695214e0_f64 * t3839 * t5223 - 0.11974241701863808564e0_f64 * t3851 * t5226 - 0.19957069503106347607e-1_f64 * t326 * t4928 + 0.19957069503106347607e-1_f64 * t305 * t4895 + 0.39914139006212695214e-1_f64 * t5245 * t551;
    t5248
}
