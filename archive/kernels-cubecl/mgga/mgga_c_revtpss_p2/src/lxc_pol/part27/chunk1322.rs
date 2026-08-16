//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1322/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1322<F: Float>(t12657: F, t2142: F, t1215: F, t1248: F, t12622: F, t12629: F, t1287: F, t1294: F, t26884: F, t26891: F, t26901: F, t26906: F, t26918: F, t26931: F, t26970: F, t26984: F, t26996: F, t27008: F, t27020: F, t3588: F, t3601: F, t3739: F, t3769: F, t3783: F, t3791: F, t7602: F, t7637: F, t7651: F, t7652: F, t7659: F, t7666: F, t97332: F, t97343: F, t97348: F, t97358: F, t97363: F) -> F {
    let t97370 = t12657 * t2142;
    let t97375 = -F::cast_from(0.19756347548806534796e1_f64) * t27008 * t3791 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t12622 - F::cast_from(0.13010442282307799193e1_f64) * t7659 * t26931 * t3588 * t1287 - F::cast_from(0.26020884564615598386e1_f64) * t26906 * t97332 * t3601 * t3769 + F::cast_from(0.13010442282307799193e1_f64) * t26906 * t26931 * t3601 * t3783 - F::cast_from(0.13010442282307799193e1_f64) * t26918 * t26901 + F::cast_from(0.10408353825846239354e2_f64) * t97343 * t26996 - F::cast_from(0.78062653693846795158e1_f64) * t97348 * t26970 * t1248 * t1287 + F::cast_from(0.26020884564615598386e1_f64) * t7651 * t7652 * t26884 * t1294 + F::cast_from(0.10408353825846239354e2_f64) * t97358 * t7637 * t2142 * t12629 - F::cast_from(0.52041769129231196772e1_f64) * t97363 * t26891 + F::cast_from(0.39512695097613069591e1_f64) * t27008 * t3739 - F::cast_from(0.26020884564615598386e1_f64) * t26984 * t7666 - F::cast_from(0.19756347548806534796e1_f64) * t97370 * t1215 + F::cast_from(0.39512695097613069591e1_f64) * t27020 * t3739;
    t97375
}
