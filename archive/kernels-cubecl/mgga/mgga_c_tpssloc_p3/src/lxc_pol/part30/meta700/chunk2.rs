//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2255/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2255<F: Float>(t16918: F, t23146: F, t16898: F, t4191: F, t87199: F, t4240: F, t232: F, t58569: F, t6605: F, t815: F, t2628: F, t5585: F, t828: F) -> (F, F, F, F, F, F) {
    let t98847 = t23146 * t16918;
    let t98849 = t23146 * t16898;
    let t98851 = t87199 * t4191;
    let t98853 = t87199 * t4240;
    let t98858 = t6605 * t815 * t58569 * t232;
    let t98862 = t6605 * t2628 * t5585 * t828;
    (t98847, t98849, t98851, t98853, t98858, t98862)
}
