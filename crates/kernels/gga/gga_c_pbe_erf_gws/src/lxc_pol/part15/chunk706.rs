//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 706/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk706<F: Float>(t4713: F, t1290: F, t155: F, t1294: F, t174: F, t1258: F, t331: F, t434: F, t456: F, t4607: F, t1318: F, t448: F, t75: F, t1321: F, t83: F, t1224: F, t1229: F, t1232: F, t1254: F, t1262: F, t1300: F, t1305: F, t1315: F, t1320: F, t1323: F, t435: F, t442: F, t457: F, t4664: F, t4674: F, t4679: F, t4682: F, t4688: F, t4689: F, t4697: F, t4701: F, t4711: F) -> (F, F, F, F, F) {
    let t4714 = 0.53425e-1 * t4713;
    let t4715 = t155 * t1290;
    let t4717 = t174 * t4715 * t1294;
    let t4718 = 0.85917146441092277512e0 * t4717;
    let t4719 = t155 * t1258;
    let t4723 = t331 * t434;
    let t4730 = t4607 * t456;
    let t4734 = 1.0 / t1318 / t448;
    let t4735 = t75 * t4734;
    let t4737 = 1.0 / t1321 / t83;
    let t4738 = t4607 * t4737;
    let t4741 = -t4664 + 1.0 * t435 * t4674 + 0.20691336878655965246e4 * t4679 * t4682 - t4688 + 0.32530742648344572643e-1 * t174 * t4689 * t1305 + 0.10274e0 * t174 * t155 * t1229 * t1232 - 0.48159446095139119799e0 * t174 * t4697 * t1323 + 0.21687161765563048428e-1 * t174 * t4701 * t457 - 0.16265371324172286321e-1 * t174 * t1300 * t1315 - t4711 + t4714 + t4718 - 0.16522997748472177549e1 * t174 * t4719 * t1262 + 0.68493333333333333332e-1 * t174 * t4723 * t442 - 0.51369999999999999999e-1 * t174 * t1224 * t1254 + 0.35089340384731224426e1 * t1320 * t4730 + 0.1025389702100779493e4 * t4735 * t4738;
    (t4714, t4718, t4734, t4737, t4741)
}
