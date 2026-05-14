//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1027/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1027<F: Float>(t322: F, t42615: F, t42646: F, t42677: F, t42709: F, t42742: F, t42774: F, t42807: F, t1065: F, t3229: F, t11002: F, t3269: F, t1039: F, t11862: F, t12580: F, t374: F, t42441: F, t42443: F, t42447: F, t42450: F, t42452: F, t42457: F, t42460: F, t42462: F, t42465: F, t42467: F, t42471: F, t42475: F, t860: F) -> (F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t42809 = piecewise5(t323, t42615, t331, t42646 + t42677 + t42709 + t42742, t42774 + t42807);
    let t42811 = t1065 * t3229;
    let t42812 = t11002 * t42811;
    let t42814 = 5.0 / 16.0 * t3269 * t42812;
    let t42815 = 2.0 * t1039 * t11862 + t12580 * t860 + t374 * t42809 + t42441 + t42443 - t42447 + t42450 - t42452 - t42457 + t42460 + t42462 - t42465 - t42467 + t42471 - t42475 - t42814;
    (t42814, t42815)
}
