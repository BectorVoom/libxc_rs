//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1145/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1145<F: Float>(t37841: F, t2833: F, t545: F, t3300: F, t10875: F, t11744: F, t146: F, t2206: F, t2832: F, t3305: F, t10760: F, t22820: F, t25697: F) -> (F, F, F, F, F) {
    let t39738 = F::new(0.13506635798907349461e1) * t37841;
    let t39739 = t545 * t2833;
    let t39740 = t39739 * t3300;
    let t39742 = t11744 * t10875;
    let t39745 = t146 * t2206 * t2832;
    let t39746 = t39745 * t3305;
    let t39749 = t22820 * t10760 * t25697;
    (t39738, t39740, t39742, t39746, t39749)
}
