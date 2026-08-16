//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1132/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1132<F: Float>(t10907: F, t2201: F, t3602: F, t11824: F, t2207: F, t3336: F, t10781: F, t7970: F, t2553: F, t37764: F, t11693: F, t6205: F) -> (F, F, F, F, F) {
    let t39569 = t2201 * t10907 * t3602;
    let t39572 = t2207 * t3336 * t11824;
    let t39577 = t10781 * t7970;
    let t39579 = t37764 * t2553;
    let t39580 = F::cast_from(0.25610080155860322884e0_f64) * t39579;
    let t39581 = t6205 * t11693;
    (t39569, t39572, t39577, t39580, t39581)
}
