//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 909/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk909(t12820: f64, t484: f64, t1063: f64, t31308: f64, t7937: f64, t2268: f64, t31399: f64, t2343: f64, t2787: f64, t30208: f64, t12834: f64, t6305: f64) -> (f64, f64, f64, f64, f64) {
    let t42726 = t484 * t12820;
    let t42730 = 0.34146007962811379518e0_f64 * t1063 * t7937 * t31308;
    let t42733 = 0.68292015925622759036e0_f64 * t2268 * t7937 * t31399;
    let t42737 = 0.56910013271352299198e-1_f64 * t1063 * t2343 * t2787 * t30208;
    let t42739 = 0.28455006635676149599e-1_f64 * t6305 * t12834;
    (t42726, t42730, t42733, t42737, t42739)
}
