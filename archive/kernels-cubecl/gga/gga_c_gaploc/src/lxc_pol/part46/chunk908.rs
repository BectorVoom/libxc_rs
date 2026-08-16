//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 908/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk908<F: Float>(t12820: F, t484: F, t1063: F, t31308: F, t7937: F, t2268: F, t31399: F, t2343: F, t2787: F, t30208: F, t12834: F, t6305: F) -> (F, F, F, F, F) {
    let t42726 = t484 * t12820;
    let t42730 = F::cast_from(0.34146007962811379518e0_f64) * t1063 * t7937 * t31308;
    let t42733 = F::cast_from(0.68292015925622759036e0_f64) * t2268 * t7937 * t31399;
    let t42737 = F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t2343 * t2787 * t30208;
    let t42739 = F::cast_from(0.28455006635676149599e-1_f64) * t6305 * t12834;
    (t42726, t42730, t42733, t42737, t42739)
}
