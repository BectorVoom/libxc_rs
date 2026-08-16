//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2631/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2631(t1250: f64, t21164: f64, t3720: f64, t140: f64, t6652: f64, t1222: f64, t20795: f64, t3629: f64, t3626: f64, t1261: f64, t17412: f64, t17444: f64, t17447: f64, t17453: f64, t17474: f64, t1808: f64, t21153: f64, t21157: f64, t21161: f64, t3625: f64, t3647: f64, t3718: f64, t5331: f64, t6673: f64) -> (f64, f64, f64, f64, f64) {
    let t21165 = t21164 * t1250;
    let t21166 = t3720 * t21165;
    let t21169 = t140 * t6652;
    let t21170 = t1222 * t21169;
    let t21172 = t20795 * t3629;
    let t21173 = t3626 * t21172;
    let t21176 = 0.23818898954483187207e-3_f64 * t3647 * t6673 + 0.15244095330869239812e-2_f64 * t17412 * t1808 - 0.14291339372689912324e-3_f64 * t1261 * t21153 + t17444 - t17447 - t17453 - 0.14291339372689912324e-3_f64 * t3625 * t21157 - 0.28582678745379824648e-3_f64 * t3625 * t21161 - 0.42874018118069736972e-3_f64 * t3718 * t21166 + t17474 + t21170 / 648.0_f64 + 0.14291339372689912324e-3_f64 * t5331 * t21173;
    (t21165, t21166, t21172, t21173, t21176)
}
