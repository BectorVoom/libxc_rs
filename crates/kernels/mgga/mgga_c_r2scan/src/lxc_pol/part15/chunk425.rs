//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 425/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk425<F: Float>(t377: F, t704: F, t706: F, t1762: F, t717: F, t722: F, t595: F, t766: F, t637: F, t160: F, t36: F, t164: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1763 = t377 * t704;
    let t1764 = t1763 * t706;
    let t1766 = F::cast_from(0.21687162600603479684e-1_f64) * t1762 * t1764;
    let t1767 = t377 * t717;
    let t1768 = t1767 * t722;
    let t1770 = F::cast_from(0.32106488758451047386e0_f64) * t1762 * t1768;
    let t1771 = t595 * t766;
    let t1772 = t1771 * t637;
    let t1774 = t160 * t36;
    let t1776 = F::new(132.0) * t1774 * t164;
    (t1763, t1764, t1766, t1767, t1768, t1770, t1771, t1772, t1774, t1776)
}
