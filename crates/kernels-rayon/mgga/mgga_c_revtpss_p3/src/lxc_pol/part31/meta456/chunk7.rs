//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1656/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1656(t21040: f64, t3629: f64, t3626: f64, t12840: f64, t20795: f64, t1222: f64, t1227: f64, t13012: f64, t17593: f64, t17619: f64, t17622: f64, t21200: f64, t21203: f64, t21210: f64, t21213: f64, t21216: f64, t3625: f64, t5340: f64, t5369: f64, t5373: f64, t5384: f64, t5386: f64) -> f64 {
    let t21218 = t21040 * t3629;
    let t21219 = t3626 * t21218;
    let t21222 = t20795 * t12840;
    let t21223 = t3626 * t21222;
    let t21226 = t17593 + 0.85748036236139473944e-3_f64 * t5384 * t21200 - 0.45732285992607719436e-2_f64 * t21203 * t5386 + t13012 / 1296.0_f64 - t17619 - t17622 + t5373 * t5369 / 54.0_f64 - t1222 * t21210 / 288.0_f64 - 11.0_f64 / 324.0_f64 * t21213 * t1227 - 0.19055119163586549765e-3_f64 * t21216 - 0.14291339372689912324e-3_f64 * t3625 * t21219 - 0.28582678745379824648e-3_f64 * t5340 * t21223;
    t21226
}
