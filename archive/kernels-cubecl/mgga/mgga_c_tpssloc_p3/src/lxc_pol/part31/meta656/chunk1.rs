//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1939/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1939<F: Float>(t5587: F, t81803: F, t1512: F, t87295: F, t23097: F, t232: F, t67793: F, t815: F, t2628: F, t5585: F, t776: F, t13228: F, t4233: F, t6605: F) -> (F, F, F, F, F) {
    let t98752 = t81803 * t5587;
    let t98754 = t87295 * t1512;
    let t98758 = t23097 * t815 * t67793 * t232;
    let t98762 = t23097 * t2628 * t5585 * t776;
    let t98766 = t6605 * t2628 * t13228 * t4233;
    (t98752, t98754, t98758, t98762, t98766)
}
