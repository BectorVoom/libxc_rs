//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1944/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1944<F: Float>(t849: F, t98832: F, t23083: F, t28375: F, t28396: F, t81835: F, t58853: F, t6605: F, t828: F, t9972: F, t4250: F, t87199: F) -> (F, F, F, F, F) {
    let t98833 = t98832 * t849;
    let t98836 = t23083 * t28375;
    let t98838 = t81835 * t28396;
    let t98842 = t6605 * t9972 * t58853 * t828;
    let t98844 = t87199 * t4250;
    (t98833, t98836, t98838, t98842, t98844)
}
