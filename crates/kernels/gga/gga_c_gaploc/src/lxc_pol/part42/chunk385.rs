//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 385/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk385<F: Float>(t2580: F, t3447: F, t2508: F, t1052: F, t977: F, t3040: F, t955: F, t2976: F, t959: F, t1645: F, t948: F) -> (F, F, F, F, F, F, F) {
    let t3448 = t2580 * t3447;
    let t3450 = F::new(0.15381052460284448567e-1) * t2508 * t3448;
    let t3459 = t1052 * t977;
    let t3463 = F::new(0.35750489951850426669e0) * t955 * t3040;
    let t3468 = t2976 * t959;
    let t3469 = F::new(0.14896037479937677779e-1) * t3468;
    let t3470 = t1645 * t948;
    (t3448, t3450, t3459, t3463, t3468, t3469, t3470)
}
