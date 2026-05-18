//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1236/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1236<F: Float>(t26966: F, t27055: F, t26676: F, t2822: F, t7772: F, t92860: F, t27006: F, t26763: F, t2861: F, t15573: F, t27019: F, t7788: F) -> (F, F, F, F, F, F, F) {
    let t92921 = t26966 * t27055;
    let t92929 = t2822 * t26676;
    let t92931 = t7772 * t92860;
    let t92941 = t26966 * t27006;
    let t92943 = t2861 * t26763;
    let t92945 = t15573 * t27019;
    let t92946 = t7788 * t92945;
    (t92921, t92929, t92931, t92941, t92943, t92945, t92946)
}
