//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 613/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk613<F: Float>(t10241: F, t426: F, t535: F, t2268: F, t2304: F, t8195: F, t6767: F, t7937: F, t7980: F, t883: F, t2325: F, t882: F) -> (F, F, F, F) {
    let t10242 = t10241 * t426;
    let t10243 = t535 * t10242;
    let t10245 = F::new(0.28455006635676149599e-1) * t2268 * t10243;
    let t10246 = t2304 * t8195;
    let t10248 = F::new(0.19918504644973304719e0) * t2268 * t10246;
    let t10249 = t7937 * t6767;
    let t10251 = F::new(0.34146007962811379518e0) * t2268 * t10249;
    let t10252 = t883 * t7980;
    let t10253 = t2325 * t10252;
    let t10254 = t882 * t10253;
    (t10245, t10248, t10251, t10254)
}
