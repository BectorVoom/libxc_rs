//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1197/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1197<F: Float>(t481: F, t8601: F, t12428: F, t792: F, t105: F, t3055: F, t97: F, t12570: F, t42846: F, t795: F, t11496: F, t2850: F) -> (F, F, F, F, F, F) {
    let t43717 = t8601 * t481;
    let t43721 = t12428 * t792;
    let t43726 = t97 * t105 * t3055;
    let t43729 = t12570 * t792;
    let t43744 = t42846 * t795;
    let t43757 = t11496 * t2850;
    (t43717, t43721, t43726, t43729, t43744, t43757)
}
