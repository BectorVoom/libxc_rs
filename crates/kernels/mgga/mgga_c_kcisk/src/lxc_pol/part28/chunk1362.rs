//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1362/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1362<F: Float>(t22310: F, t415: F, t9687: F, t116304: F, t17004: F, t34180: F, t1636: F, t23885: F, t7242: F, t17345: F, t6714: F, t7283: F, t32903: F, t35131: F, t5054: F, t35253: F, t4811: F) -> (F, F, F, F, F, F) {
    let t121374 = t415 * t9687 * t22310;
    let t121381 = t116304 * t17004 * t34180;
    let t121385 = t7242 * t23885 * t1636;
    let t121389 = t17345 * t7283 * t6714;
    let t121399 = t5054 * t32903 * t35131;
    let t121405 = t4811 * t35253;
    (t121374, t121381, t121385, t121389, t121399, t121405)
}
