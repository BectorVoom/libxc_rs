//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1875/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1875<F: Float>(t22893: F, t23164: F, t25320: F, t1888: F, t232: F, t47528: F, t6646: F, t13398: F, t82018: F, t13404: F, t22996: F, t7521: F, t81632: F) -> (F, F, F, F, F) {
    let t87618 = t23164 * t22893 * t25320;
    let t87627 = t1888 * t6646 * t47528 * t232;
    let t87630 = t1888 * t82018 * t13398;
    let t87633 = t1888 * t22996 * t13404;
    let t87635 = t81632 * t7521;
    (t87618, t87627, t87630, t87633, t87635)
}
