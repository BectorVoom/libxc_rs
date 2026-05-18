//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1195/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1195<F: Float>(t2867: F, t3275: F, t37318: F, t10680: F, t10681: F, t10683: F, t2482: F, t10673: F, t10674: F, t10676: F, t11020: F, t11545: F) -> (F, F, F, F) {
    let t40338 = t3275 * t37318 * t2867 / F::new(4.0);
    let t40341 = t10680 * t10681 * t2482 * t10683;
    let t40342 = F::new(0.72042316457491791906e-3) * t40341;
    let t40345 = t10673 * t10674 * t2482 * t10676;
    let t40346 = F::new(0.10248087766267884742e-3) * t40345;
    let t40348 = F::new(5.0) / F::new(16.0) * t11020 * t11545;
    (t40338, t40342, t40346, t40348)
}
