//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1273/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1273<F: Float>(t37393: F, t37401: F, t39059: F, t39061: F, t39062: F, t39064: F, t43838: F, t43842: F, t44120: F, t44122: F, t44125: F, t44127: F, t44129: F, t44132: F, t44135: F) -> F {
    let t44971 = -t44120 + t44122 + F::new(0.72042316457491791901e-3) * t43838 - F::new(0.10248087766267884741e-3) * t43842 + t44125 - t44127 - t44129 - t44132 - F::new(0.86737941314158990616e-4) * t37393 - t39059 + F::new(0.92232789896410962673e-3) * t37401 + t44135 + t39061 + t39062 - t39064;
    t44971
}
