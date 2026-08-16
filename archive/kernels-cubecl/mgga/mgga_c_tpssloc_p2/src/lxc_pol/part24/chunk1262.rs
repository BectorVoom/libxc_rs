//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1262/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1262<F: Float>(t1336: F, t240: F, t80901: F, t12293: F, t12297: F, t22761: F, t12305: F, t6952: F, t12267: F, t6944: F, t1354: F, t2690: F, t6943: F) -> (F, F, F, F, F) {
    let t80903 = t1336 * t80901 * t240;
    let t80904 = t80903 * t12293;
    let t80906 = t22761 * t12297;
    let t80908 = t6952 * t12305;
    let t80910 = t12267 * t6944;
    let t80911 = t80910 * t1354;
    let t80914 = t1336 * t6943 * t2690;
    (t80904, t80906, t80908, t80911, t80914)
}
