//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 678/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk678<F: Float>(t413: F, t1253: F, t167: F, t5314: F, t1852: F, t25: F, t1251: F, t1851: F, t330: F, t829: F, t3515: F, t286: F, t287: F, t3530: F, t1262: F, t5272: F, t1260: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t418 = 0.0 < t413;
    let t5315 = t1253 * t167;
    let t5316 = t5314 * t5315;
    let t5321 = t25 * t1852;
    let t5322 = t1251 * t5321;
    let t5324 = t1851 * t330;
    let t5325 = t5324 * t829;
    let t5326 = t3515 * t5325;
    let t5329 = t286 * t287;
    let t5330 = t3530 * t1851;
    let t5331 = t5330 * t1262;
    let t5332 = t5329 * t5331;
    let t5336 = piecewise3(t418, t5272, -t5272);
    let t5337 = t1260 * t5336;
    (t5315, t5316, t5321, t5322, t5325, t5326, t5329, t5330, t5331, t5332, t5336, t5337)
}
