//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2162/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2162<F: Float>(t58300: F, t6605: F, t815: F, t25112: F, t81835: F, t232: F, t47262: F, t23097: F, t47012: F, t23083: F, t25116: F, t1510: F, t2553: F) -> (F, F, F, F, F, F) {
    let t87475 = t6605 * t815 * t58300;
    let t87477 = t81835 * t25112;
    let t87478 = F::cast_from(0.16956557559538964159e-1_f64) * t87477;
    let t87481 = t6605 * t815 * t47262 * t232;
    let t87485 = t23097 * t815 * t47012 * t232;
    let t87487 = t23083 * t25116;
    let t87488 = F::cast_from(0.28260929265898273598e-2_f64) * t87487;
    let t87491 = t23097 * t815 * t1510 * t2553;
    (t87475, t87478, t87481, t87485, t87488, t87491)
}
