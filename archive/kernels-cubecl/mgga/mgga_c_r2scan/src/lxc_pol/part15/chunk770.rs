//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 770/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk770<F: Float>(t2078: F, t784: F, t783: F, t788: F, t1607: F, t5100: F, t512: F, t6101: F, t507: F, t1591: F, t2168: F, t1584: F, t1634: F) -> (F, F, F, F, F) {
    let t6416 = t2078 * t784;
    let t6418 = t783 * t6416 * t788;
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = F::cast_from(0.174549769648958674e0_f64) * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6440 = t1584 * t1634;
    (t6418, t6420, t6424, t6425, t6440)
}
