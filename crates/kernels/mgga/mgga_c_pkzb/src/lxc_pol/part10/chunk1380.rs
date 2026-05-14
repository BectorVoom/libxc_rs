//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1380/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1380<F: Float>(t18427: F, t18430: F, t18750: F, t18887: F, t18889: F, t22230: F, t22233: F, t22236: F, t22561: F, t22578: F, t2258: F, t22662: F, t22829: F, t2297: F, t2318: F, t27262: F, t27289: F, t27295: F, t27426: F, t27429: F, t27432: F, t27436: F, t27439: F, t27443: F, t27447: F, t27472: F, t27474: F, t365: F, t3779: F, t3807: F, t3820: F, t6323: F, t8107: F, t8120: F, t8132: F, t8135: F, t8150: F, t8171: F, t8181: F, t8211: F, t889: F) -> (F,) {
    let t27609 = t27426 + t27429 - t27432 - t27436 - t27439 + 0.19964560303604640732e6 * t18887 * t3779 * t18889 * t2258 - t27443 - t27447 + 0.41016075432865626631e4 * t22578 * t22662 * t889 - 0.310907e-1 * (t18750 - 0.10654518518518518518e0 * t18427 + 0.22831111111111111111e-1 * t18430 - 0.10654518518518518518e0 * t22230 + 0.91324444444444444442e-1 * t22233 - 0.34246666666666666666e-1 * t22236 + 0.22831111111111111111e-1 * t27295 - 0.34246666666666666666e-1 * t27262 + 0.5137e-1 * t27289) * t365 - 0.4155806185363551302e3 * t22561 * t8171 - t27472 - t27474 - 0.14035736694323150897e2 * t6323 * t3807 * t2297 + 12.0 * t8120 * t8181 + 0.35089341735807877242e1 * t2318 * t3820 * t2297 + 0.70178683471615754484e1 * t8107 * t8150 - 4.0 * t8211 * t8132 - 0.38596750796862084161e3 * t22829 * t8135;
    (t27609,)
}
