//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 469/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk469<F: Float>(t2349: F, t492: F, t105: F, t1063: F, t1358: F, t2264: F, t2268: F, t2269: F, t2296: F, t2301: F, t2305: F, t2308: F, t2313: F, t2319: F, t2323: F, t2328: F, t2336: F, t2340: F, t2346: F, t380: F, t419: F, t877: F, t889: F) -> (F, F) {
    let t2350 = t492 * t2349;
    let t2353 = F::cast_from(0.37940008847568199465e-1_f64) * t380 * t877 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t877 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t2264 + F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t2269 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t2296 - F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t2301 - F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t2305 - F::cast_from(0.15808337019820083111e-2_f64) * t2308 + F::cast_from(0.11856252764865062333e-2_f64) * t2313 - F::cast_from(0.11856252764865062333e-2_f64) * t2319 + F::cast_from(0.11856252764865062333e-2_f64) * t2323 - F::cast_from(0.11856252764865062333e-2_f64) * t2328 - F::cast_from(0.37940008847568199465e-1_f64) * t380 * t889 - F::cast_from(0.28455006635676149599e-1_f64) * t419 * t889 + F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t2336 + F::cast_from(0.31616674039640166221e-2_f64) * t1358 * t2340 + F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t2346 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t2350;
    (t2350, t2353)
}
