//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 972/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk972<F: Float>(t1063: F, t2343: F, t2787: F, t30208: F, t12834: F, t6305: F, t2268: F, t9493: F, t988: F, t12763: F, t6313: F, t12770: F, t2312: F) -> (F, F, F, F, F) {
    let t42737 = F::new(0.56910013271352299198e-1) * t1063 * t2343 * t2787 * t30208;
    let t42739 = F::new(0.28455006635676149599e-1) * t6305 * t12834;
    let t42742 = F::new(0.28455006635676149599e-1) * t2268 * t9493 * t988;
    let t42743 = t6313 * t12763;
    let t42745 = t2312 * t12770;
    (t42737, t42739, t42742, t42743, t42745)
}
