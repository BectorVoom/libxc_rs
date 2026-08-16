//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1913/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1913<F: Float>(t1351: F, t1992: F, t5318: F, t550: F, t6976: F, t16036: F, t22633: F, t3807: F, t12407: F, t5335: F, t22704: F, t22705: F, t5345: F) -> (F, F, F, F) {
    let t90770 = t1992 * t6976 * t5318 * t1351 * t550;
    let t90774 = t22633 * t6976 * t16036 * t3807;
    let t90778 = t22633 * t6976 * t5335 * t12407;
    let t90781 = t22704 * t22705 * t5345;
    (t90770, t90774, t90778, t90781)
}
