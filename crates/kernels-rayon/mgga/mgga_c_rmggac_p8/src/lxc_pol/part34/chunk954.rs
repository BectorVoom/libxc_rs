//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 954/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk954(t76924: f64, t14672: f64, t17859: f64, t74219: f64, t14551: f64, t8368: f64, t74232: f64, t74199: f64, t74207: f64, t74209: f64, t74213: f64, t74217: f64, t74225: f64, t74228: f64, t74235: f64, t76904: f64, t76913: f64, t76918: f64, t76923: f64) -> f64 {
    let t76925 = 0.42564599893297839398e-5_f64 * t76924;
    let t76926 = t17859 * t14672;
    let t76927 = 0.12769379967989351819e-4_f64 * t76926;
    let t76928 = 0.1921128438866447784e-2_f64 * t74219;
    let t76930 = t8368 * t14551;
    let t76931 = 0.90915538847484472429e-2_f64 * t76930;
    let t76932 = 0.68186654135613354325e-2_f64 * t74232;
    let t76934 = -0.57000320883372412496e-7_f64 * t74199 + t76904 + 0.58171619854173713846e-5_f64 * t74207 - 0.58171619854173713846e-5_f64 * t74209 + 0.58171619854173713846e-5_f64 * t74213 - 0.17451485956252114154e-4_f64 * t74217 - t76913 + t76918 + t76923 + t76925 - t76927 + t76928 - t74225 + 0.70077224371605468752e-6_f64 * t74228 + t76931 - t76932 + 0.35038612185802734376e-6_f64 * t74235;
    t76934
}
