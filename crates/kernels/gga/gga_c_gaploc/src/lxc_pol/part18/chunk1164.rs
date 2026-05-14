//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1164/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1164<F: Float>(t33507: F, t11061: F, t14549: F, t32356: F, t5241: F, t5640: F, t590: F, t1890: F, t1966: F, t32435: F, t326: F, t32889: F, t7394: F, t28412: F, t8970: F, t913: F) -> (F, F, F, F, F, F) {
    let t33508 = 0.2698205900461089792e0 * t33507;
    let t33518 = 0.30674340763136599742e1 * t14549 * t11061;
    let t33522 = 0.30674340763136599742e1 * t5640 * t5241 * t32356 * t590;
    let t33526 = 0.51123901271894332902e1 * t1966 * t1890 * t32435 * t590;
    let t33529 = 0.92023022289409799224e1 * t7394 * t326 * t32889;
    let t33531 = t28412 * t913 * t8970;
    (t33508, t33518, t33522, t33526, t33529, t33531)
}
