//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1923/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1923<F: Float>(t22633: F, t26421: F, t3856: F, t6976: F, t26462: F, t6914: F, t22705: F, t26414: F, t81228: F, t26415: F, t81159: F, t3851: F) -> (F, F, F, F, F) {
    let t90933 = t22633 * t6976 * t26421 * t3856;
    let t90956 = t6914 * t26462;
    let t90961 = t81228 * t22705 * t26414;
    let t90963 = t81159 * t26415;
    let t90968 = t22633 * t6976 * t26421 * t3851;
    (t90933, t90956, t90961, t90963, t90968)
}
