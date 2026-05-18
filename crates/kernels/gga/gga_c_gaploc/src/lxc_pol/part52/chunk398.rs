//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 398/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk398<F: Float>(t3541: F, t531: F, t3545: F, t3516: F, t569: F, t568: F, t600: F, t3529: F, t1565: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3570 = t531 * t3541;
    let t3573 = t531 * t3545;
    let t3576 = t569 * t3516;
    let t3577 = t568 * t3576;
    let t3581 = t600 * t3516;
    let t3582 = t568 * t3581;
    let t3585 = t569 * t3529;
    let t3586 = t568 * t3585;
    let t3591 = t1565 * t3516;
    let t3592 = t568 * t3591;
    let t3595 = t600 * t3529;
    let t3596 = t568 * t3595;
    (t3570, t3573, t3576, t3577, t3581, t3582, t3585, t3586, t3591, t3592, t3595, t3596)
}
