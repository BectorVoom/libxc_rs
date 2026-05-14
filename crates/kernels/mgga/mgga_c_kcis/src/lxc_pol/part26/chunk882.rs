//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 882/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk882<F: Float>(t1897: F, t1961: F, t1419: F, t16387: F, t11634: F, t1319: F, t21624: F, t1477: F, t21267: F, t542: F, t3255: F, t7238: F, t1409: F, t7122: F, t3786: F, t7123: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22114 = t1897 * t1961;
    let t22116 = t16387 * t22114 * t1419;
    let t22120 = t11634 * t21624 * t1319;
    let t22127 = t1477 * t21267;
    let t22128 = t542 * t22127;
    let t22131 = t3255 * t7238;
    let t22133 = t1409 * t7122;
    let t22134 = t22133 * t1319;
    let t22135 = t3786 * t22134;
    let t22138 = t7123 * t1419;
    (t22114, t22116, t22120, t22127, t22128, t22131, t22134, t22135, t22138)
}
