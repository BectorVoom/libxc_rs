//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 714/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk714<F: Float>(t13728: F, t1457: F, t1572: F, t12068: F, t874: F, t1445: F, t1562: F, t13750: F, t531: F, t557: F, t13749: F, t569: F) -> (F, F, F, F, F, F, F, F) {
    let t13810 = t1457 * t13728;
    let t13811 = t1572 * t13810;
    let t13813 = t12068 * t874;
    let t13814 = t1445 * t13813;
    let t13815 = t1562 * t13814;
    let t13818 = t531 * t13750;
    let t13820 = F::new(0.35750489951850426669e0) * t557 * t13818;
    let t13821 = t569 * t13749;
    (t13810, t13811, t13813, t13814, t13815, t13818, t13820, t13821)
}
