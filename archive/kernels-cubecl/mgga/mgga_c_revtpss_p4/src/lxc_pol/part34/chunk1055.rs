//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1055/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1055<F: Float>(t24493: F, t3523: F, t1196: F, t1179: F, t1188: F, t24407: F, t1832: F, t6752: F, t1828: F, t3737: F, t6744: F, t1774: F) -> (F, F, F, F, F) {
    let t24494 = t24493 * t3523;
    let t24496 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t24494;
    let t24498 = t1179 * t24407 * t1188;
    let t24500 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t24498;
    let t24501 = t6752 * t1832;
    let t24509 = t3737 * t1828 * t6744;
    let t24514 = t1774 * t6744;
    (t24496, t24500, t24501, t24509, t24514)
}
