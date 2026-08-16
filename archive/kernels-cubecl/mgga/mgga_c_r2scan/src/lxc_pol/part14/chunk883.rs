//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 883/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk883<F: Float>(t551: F, t552: F, t8057: F, t2155: F, t7407: F, t2609: F, t6395: F, t113: F, t7433: F) -> (F, F, F, F) {
    let t8059 = t551 * t552 * t8057;
    let t8062 = t2155 * t7407;
    let t8065 = F::cast_from(0.11643651550782197811e-1_f64) * t6395 * t2609;
    let t8066 = t7433 * t113;
    (t8059, t8062, t8065, t8066)
}
