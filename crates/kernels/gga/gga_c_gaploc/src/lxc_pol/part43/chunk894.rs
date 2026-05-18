//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 894/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk894<F: Float>(t43107: F, t688: F, t2508: F, t779: F, t1897: F, t27997: F, t3009: F, t7226: F, t28013: F, t3276: F, t8670: F, t2541: F, t33680: F) -> (F, F, F, F, F) {
    let t43108 = t43107 * t688;
    let t43111 = F::new(0.76905262301422242837e-2) * t2508 * t779 * t43108;
    let t43115 = F::new(0.46143157380853345701e-1) * t1897 * t7226 * t3009 * t27997;
    let t43119 = F::new(0.92286314761706691402e-1) * t2508 * t7226 * t3009 * t28013;
    let t43122 = F::new(0.53833683610995569986e-1) * t1897 * t3276 * t8670;
    let t43125 = F::new(0.10766736722199113997e0) * t2508 * t2541 * t33680;
    (t43111, t43115, t43119, t43122, t43125)
}
