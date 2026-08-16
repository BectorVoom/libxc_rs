//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1814/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1814<F: Float>(t23030: F, t23253: F, t23204: F, t23241: F, t81640: F, t23273: F, t81591: F, t23228: F, t6555: F, t81573: F, t6563: F, t81597: F) -> (F, F, F, F, F) {
    let t82099 = t23030 * t23253;
    let t82108 = t81640 * t23204 * t23241;
    let t82115 = t81591 * t23273;
    let t82120 = t81573 * t23228 * t6555;
    let t82122 = t81597 * t6563;
    (t82099, t82108, t82115, t82120, t82122)
}
