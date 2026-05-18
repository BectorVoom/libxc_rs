//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 372/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk372<F: Float>(t2386: F, t600: F, t1678: F, t1681: F, t2366: F, t2373: F, t2376: F, t2379: F) -> (F, F) {
    let t2387 = t2386 * t600;
    let t2394 = F::new(0.258925e1) * t2373 - t1678 - F::new(0.301925e0) * t2366 + F::new(0.16504875e0) * t2376 - t1681 - F::new(0.82785e-1) * t2379;
    (t2387, t2394)
}
