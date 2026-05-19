//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 340/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk340<F: Float>(t1066: F, t1445: F, t1394: F, t1398: F, t1403: F, t1407: F, t1412: F, t1416: F, t1417: F, t1421: F, t1424: F, t1429: F, t1430: F, t1436: F, t1438: F, t1441: F, t1442: F, t547: F, t552: F, t567: F, t587: F, t591: F) -> F {
    let t1446 = t1445 * t1066;
    let t1449 = -F::cast_from(0.5680433474654925878e-1_f64) * t587 * t1394 - F::cast_from(0.79445533226334281486e-1_f64) * t1398 * t552 + F::cast_from(0.92686455430723328401e-1_f64) * t547 * t1403 + F::cast_from(0.51123901271894332902e0_f64) * t1407 * t591 - F::cast_from(0.2556195063594716645e0_f64) * t587 * t1412 + F::cast_from(0.79445533226334281486e-1_f64) * t1416 * t1417 - F::cast_from(0.79445533226334281486e-1_f64) * t1421 * t1424 + F::cast_from(0.79445533226334281486e-1_f64) * t1429 * t1430 - F::cast_from(0.51123901271894332902e0_f64) * t1436 * t1438 + F::cast_from(0.1022478025437886658e1_f64) * t1441 * t1442 + F::cast_from(0.46011511144704899612e1_f64) * t567 * t1446;
    t1449
}
