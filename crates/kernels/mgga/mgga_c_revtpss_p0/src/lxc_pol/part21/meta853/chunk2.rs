//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3214/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3214<F: Float>(t17583: F, t3172: F, t3711: F, t1042: F, t1252: F, t1261: F, t12621: F, t12889: F, t1469: F, t17550: F, t17693: F, t1803: F, t225: F, t3674: F, t45382: F, t45389: F, t480: F, t484: F, t5296: F, t53450: F, t56479: F, t57622: F, t59337: F, t59339: F, t59349: F, t59351: F, t59353: F, t59355: F, t59358: F, t59360: F, t59362: F, t59371: F, t59375: F, t59379: F) -> F {
    let t59386 = t3711 * t3172 * t17583;
    let t59388 = t59337 - t59339 - F::cast_from(0.11433071498151929859e-2_f64) * t12889 * t1803 * t484 + F::cast_from(0.21437009059034868486e-3_f64) * t56479 * t225 * t480 * t484 - F::cast_from(0.85748036236139473944e-3_f64) * t45382 + F::cast_from(0.85748036236139473944e-3_f64) * t45389 - F::cast_from(0.42874018118069736972e-3_f64) * t59349 - F::cast_from(0.42874018118069736972e-3_f64) * t59351 + F::cast_from(0.85748036236139473944e-3_f64) * t59353 - F::cast_from(0.68598428988911579154e-2_f64) * t59355 * t3674 + F::cast_from(0.45732285992607719436e-2_f64) * t59358 + F::cast_from(0.85748036236139473944e-3_f64) * t59360 - F::cast_from(0.19055119163586549765e-2_f64) * t17693 * t59362 * t57622 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t5296 * t1469 * t12621 - F::cast_from(0.34299214494455789577e-2_f64) * t59371 * t1252 + F::cast_from(0.64311027177104605458e-3_f64) * t59375 * t1252 + F::cast_from(0.42874018118069736972e-3_f64) * t59379 + F::cast_from(0.42874018118069736973e-2_f64) * t1261 * t1042 * t17550 * t53450 + F::cast_from(0.28582678745379824648e-3_f64) * t59386;
    t59388
}
