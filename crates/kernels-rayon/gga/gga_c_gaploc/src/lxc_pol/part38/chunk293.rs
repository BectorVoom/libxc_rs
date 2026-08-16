//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 293/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk293(t2349: f64, t492: f64, t105: f64, t1063: f64, t1358: f64, t2264: f64, t2268: f64, t2269: f64, t2296: f64, t2301: f64, t2305: f64, t2308: f64, t2313: f64, t2319: f64, t2323: f64, t2328: f64, t2336: f64, t2340: f64, t2346: f64, t380: f64, t419: f64, t877: f64, t889: f64) -> f64 {
    let t2350 = t492 * t2349;
    let t2353 = 0.37940008847568199465e-1_f64 * t380 * t877 + 0.28455006635676149599e-1_f64 * t419 * t877 - 0.28455006635676149599e-1_f64 * t1063 * t2264 + 0.28455006635676149599e-1_f64 * t2268 * t2269 + 0.28455006635676149599e-1_f64 * t105 * t2296 - 0.31616674039640166221e-2_f64 * t1358 * t2301 - 0.85365019907028448797e-1_f64 * t2268 * t2305 - 0.15808337019820083111e-2_f64 * t2308 + 0.11856252764865062333e-2_f64 * t2313 - 0.11856252764865062333e-2_f64 * t2319 + 0.11856252764865062333e-2_f64 * t2323 - 0.11856252764865062333e-2_f64 * t2328 - 0.37940008847568199465e-1_f64 * t380 * t889 - 0.28455006635676149599e-1_f64 * t419 * t889 + 0.28455006635676149599e-1_f64 * t1063 * t2336 + 0.31616674039640166221e-2_f64 * t1358 * t2340 + 0.56910013271352299198e-1_f64 * t2268 * t2346 - 0.28455006635676149599e-1_f64 * t105 * t2350;
    t2353
}
