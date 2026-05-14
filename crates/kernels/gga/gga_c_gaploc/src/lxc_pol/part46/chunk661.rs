//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 661/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk661<F: Float>(t1959: F, t2590: F, t19531: F, t486: F, t169: F, t18310: F, t1381: F, t2353: F, t2967: F, t10007: F, t8502: F, t10012: F, t8669: F, t2101: F, t2925: F, t313: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23575 = t2590 * t1959;
    let t23915 = t19531 * t486;
    let t24139 = t18310 * t169;
    let t24215 = t2353 * t1381;
    let t24295 = t2967 * t1959;
    let t24501 = t10007 * t8502;
    let t24505 = t10012 * t8502;
    let t24549 = t10012 * t8669;
    let t24660 = t2101 * t2925;
    let t24661 = t313 * t24660;
    (t23575, t23915, t24139, t24215, t24295, t24501, t24505, t24549, t24660, t24661)
}
