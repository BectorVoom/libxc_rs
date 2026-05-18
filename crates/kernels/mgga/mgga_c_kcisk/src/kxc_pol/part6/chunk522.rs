//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 522/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk522<F: Float>(t1413: F, t2257: F, t1556: F, t2306: F, t260: F, t338: F, t67: F, t41: F, t4143: F, t1576: F, t2318: F, t2317: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t6387 = t2257 * t1413;
    let t6388 = t6387 * sigma0;
    let t6426 = t2306 * t1556;
    let t6442 = t260 * t67 * t338;
    let t6443 = t41 * t4143;
    let t6456 = t2318 * t1576;
    let t6458 = t2317 * sigma0;
    (t6387, t6388, t6426, t6442, t6443, t6456, t6458)
}
