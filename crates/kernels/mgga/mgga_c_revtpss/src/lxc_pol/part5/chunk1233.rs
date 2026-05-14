//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1233/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1233<F: Float>(t1770: F, t5477: F, t1248: F, t17847: F, t20956: F, t17854: F, t1280: F, t20721: F, t5284: F, t5464: F, t5332: F, t1287: F, t20856: F, t1794: F, t5412: F, t5245: F, t5486: F) -> (F, F, F, F, F, F, F, F) {
    let t21579 = t1770 * t5477;
    let t21582 = t17847 * t1248;
    let t21583 = t20956 * t21582;
    let t21586 = t17854 * t1248;
    let t21587 = t20956 * t21586;
    let t21592 = t1280 * t20721;
    let t21595 = t5464 * t5284;
    let t21596 = t5332 * t21595;
    let t21599 = t20856 * t1287;
    let t21607 = t5412 * t1794 * t1287;
    let t21610 = t5486 * t5245;
    (t21579, t21583, t21587, t21592, t21596, t21599, t21607, t21610)
}
