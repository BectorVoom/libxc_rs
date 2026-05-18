//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1147/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1147<F: Float>(t2096: F, t2665: F, t565: F, t10711: F, t11696: F, t37936: F, t10710: F, t10728: F, t24902: F, t11699: F, t37939: F, t1592: F, t25172: F, t3308: F) -> (F, F, F, F, F, F) {
    let t39960 = t2665 * t2096;
    let t39961 = t565 * t39960;
    let t39962 = t39961 * t10711;
    let t39964 = t37936 * t11696;
    let t39967 = t10728 * t10710 * t24902;
    let t39969 = t37939 * t11699;
    let t39972 = t1592 * t3308 * t25172;
    (t39960, t39962, t39964, t39967, t39969, t39972)
}
