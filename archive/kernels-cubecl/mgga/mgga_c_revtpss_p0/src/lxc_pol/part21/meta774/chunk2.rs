//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2752/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2752<F: Float>(t10777: F, t10779: F, t2749: F, t50412: F, t14686: F, t837: F, t40593: F, t4452: F, t14671: F, t2646: F, t4343: F, t836: F) -> (F, F, F, F, F) {
    let t50628 = t10777 * t10779 * t50412 * t2749;
    let t50632 = t10777 * t14686 * t50412 * t837;
    let t50634 = t40593 * t4452;
    let t50643 = t10777 * t14686 * t14671 * t2646;
    let t50649 = t4343 * t836;
    (t50628, t50632, t50634, t50643, t50649)
}
