//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3214/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3214(t17583: f64, t3172: f64, t3711: f64, t1042: f64, t1252: f64, t1261: f64, t12621: f64, t12889: f64, t1469: f64, t17550: f64, t17693: f64, t1803: f64, t225: f64, t3674: f64, t45382: f64, t45389: f64, t480: f64, t484: f64, t5296: f64, t53450: f64, t56479: f64, t57622: f64, t59337: f64, t59339: f64, t59349: f64, t59351: f64, t59353: f64, t59355: f64, t59358: f64, t59360: f64, t59362: f64, t59371: f64, t59375: f64, t59379: f64) -> f64 {
    let t59386 = t3711 * t3172 * t17583;
    let t59388 = t59337 - t59339 - 0.11433071498151929859e-2_f64 * t12889 * t1803 * t484 + 0.21437009059034868486e-3_f64 * t56479 * t225 * t480 * t484 - 0.85748036236139473944e-3_f64 * t45382 + 0.85748036236139473944e-3_f64 * t45389 - 0.42874018118069736972e-3_f64 * t59349 - 0.42874018118069736972e-3_f64 * t59351 + 0.85748036236139473944e-3_f64 * t59353 - 0.68598428988911579154e-2_f64 * t59355 * t3674 + 0.45732285992607719436e-2_f64 * t59358 + 0.85748036236139473944e-3_f64 * t59360 - 0.19055119163586549765e-2_f64 * t17693 * t59362 * t57622 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t5296 * t1469 * t12621 - 0.34299214494455789577e-2_f64 * t59371 * t1252 + 0.64311027177104605458e-3_f64 * t59375 * t1252 + 0.42874018118069736972e-3_f64 * t59379 + 0.42874018118069736973e-2_f64 * t1261 * t1042 * t17550 * t53450 + 0.28582678745379824648e-3_f64 * t59386;
    t59388
}
