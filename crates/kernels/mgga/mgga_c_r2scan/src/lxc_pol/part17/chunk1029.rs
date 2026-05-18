//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1029/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1029<F: Float>(t11368: F, t11372: F, t11374: F, t11375: F, t11377: F, t11378: F, t11379: F, t11616: F, t12752: F, t12815: F, t12817: F, t12203: F, t3579: F) -> (F, F) {
    let t12942 = t11368 + t11372 - t11374 + t12752 - t12815 + t12817 + t11375 + t11377 - t11378 - F::new(0.162600798888400151e-2) * t11616 + t11379;
    let t12943 = t3579 * t12203;
    (t12942, t12943)
}
