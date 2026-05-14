//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1160/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1160<F: Float>(t3596: F, t7627: F, t26936: F, t3566: F, t13181: F, t3140: F, t1243: F, t2149: F, t2147: F, t44841: F, t7635: F, t3572: F, t8945: F, t12657: F, t2142: F, t1215: F, t1248: F, t12622: F, t12629: F, t1287: F, t1294: F, t26884: F, t26891: F, t26901: F, t26906: F, t26918: F, t26931: F, t26970: F, t26984: F, t26996: F, t27008: F, t27020: F, t3588: F, t3601: F, t3739: F, t3769: F, t3783: F, t3791: F, t7602: F, t7637: F, t7651: F, t7652: F, t7659: F, t7666: F) -> (F,) {
    let t97332 = t3596 * t7627;
    let t97343 = t3566 * t26936;
    let t97346 = t3140 * t13181;
    let t97348 = t2149 * t97346 * t1243;
    let t97358 = t2147 * t44841 * t7635;
    let t97363 = t3572 * t8945;
    let t97370 = t12657 * t2142;
    let t97375 = -0.19756347548806534796e1 * t27008 * t3791 - 0.65854491829355115987e0 * t7602 * t12622 - 0.13010442282307799193e1 * t7659 * t26931 * t3588 * t1287 - 0.26020884564615598386e1 * t26906 * t97332 * t3601 * t3769 + 0.13010442282307799193e1 * t26906 * t26931 * t3601 * t3783 - 0.13010442282307799193e1 * t26918 * t26901 + 0.10408353825846239354e2 * t97343 * t26996 - 0.78062653693846795158e1 * t97348 * t26970 * t1248 * t1287 + 0.26020884564615598386e1 * t7651 * t7652 * t26884 * t1294 + 0.10408353825846239354e2 * t97358 * t7637 * t2142 * t12629 - 0.52041769129231196772e1 * t97363 * t26891 + 0.39512695097613069591e1 * t27008 * t3739 - 0.26020884564615598386e1 * t26984 * t7666 - 0.19756347548806534796e1 * t97370 * t1215 + 0.39512695097613069591e1 * t27020 * t3739;
    (t97375,)
}
