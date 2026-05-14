//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 761/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk761<F: Float>(t42717: F, t39731: F, t2321: F, t34600: F, t9074: F, t1063: F, t31308: F, t7937: F, t2268: F, t31399: F, t2343: F, t2787: F, t30208: F, t12834: F, t6305: F, t9493: F, t988: F) -> (F, F, F, F, F, F, F, F) {
    let t42718 = 0.47425011059460249332e-2 * t42717;
    let t42719 = 0.23712505529730124666e-2 * t39731;
    let t42721 = t9074 * t34600 * t2321;
    let t42722 = 0.23712505529730124666e-2 * t42721;
    let t42730 = 0.34146007962811379518e0 * t1063 * t7937 * t31308;
    let t42733 = 0.68292015925622759036e0 * t2268 * t7937 * t31399;
    let t42737 = 0.56910013271352299198e-1 * t1063 * t2343 * t2787 * t30208;
    let t42739 = 0.28455006635676149599e-1 * t6305 * t12834;
    let t42742 = 0.28455006635676149599e-1 * t2268 * t9493 * t988;
    (t42718, t42719, t42722, t42730, t42733, t42737, t42739, t42742)
}
