//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 338/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk338(t1066: f64, t1445: f64, t1394: f64, t1398: f64, t1403: f64, t1407: f64, t1412: f64, t1416: f64, t1417: f64, t1421: f64, t1424: f64, t1429: f64, t1430: f64, t1436: f64, t1438: f64, t1441: f64, t1442: f64, t547: f64, t552: f64, t567: f64, t587: f64, t591: f64) -> f64 {
    let t1446 = t1445 * t1066;
    let t1449 = -0.5680433474654925878e-1_f64 * t587 * t1394 - 0.79445533226334281486e-1_f64 * t1398 * t552 + 0.92686455430723328401e-1_f64 * t547 * t1403 + 0.51123901271894332902e0_f64 * t1407 * t591 - 0.2556195063594716645e0_f64 * t587 * t1412 + 0.79445533226334281486e-1_f64 * t1416 * t1417 - 0.79445533226334281486e-1_f64 * t1421 * t1424 + 0.79445533226334281486e-1_f64 * t1429 * t1430 - 0.51123901271894332902e0_f64 * t1436 * t1438 + 0.1022478025437886658e1_f64 * t1441 * t1442 + 0.46011511144704899612e1_f64 * t567 * t1446;
    t1449
}
