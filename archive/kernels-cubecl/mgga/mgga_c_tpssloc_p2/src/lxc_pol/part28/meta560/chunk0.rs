//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1832/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1832<F: Float>(t13487: F, t86721: F, t22960: F, t58071: F, t2: F, t2752: F, t584: F, t868: F, t4303: F, t606: F, t870: F, t776: F) -> (F, F, F, F, F) {
    let t86722 = t86721 * t13487;
    let t86727 = t22960 * t58071;
    let t86730 = t2752 * t2;
    let t86732 = t86730 * t584 * t868;
    let t86746 = t606 * t4303;
    let t86753 = t870 * t2;
    let t86755 = t86753 * t584 * t776;
    (t86722, t86727, t86732, t86746, t86755)
}
