//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1329/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1329<F: Float>(t5728: F, t759: F, t2916: F, t21604: F, t21686: F, t21841: F, t21852: F, t21862: F, t21867: F, t21870: F, t21874: F, t21877: F, t26378: F, t301: F, t5729: F, t5956: F, t757: F, t758: F, t761: F, t7664: F, t7707: F, t7736: F, t7742: F, t9277: F, t9311: F, t9316: F, t9321: F) -> (F, F) {
    let t26387 = t5728 * t759;
    let t26392 = t2916 * t759;
    let t26411 = 0.21437009059034868486e-3 * t757 * t758 * t301 * t26378 * t761 - 0.30488190661738479624e-2 * t21841 + 0.17149607247227894789e-2 * t21852 + 0.3811023832717309953e-3 * t21862 + 0.85748036236139473944e-3 * t7664 * t21686 * t26387 * t9277 + 0.51448821741683684367e-2 * t7736 * t21686 * t5956 * t26392 - 0.51448821741683684367e-2 * t7742 * t21686 * t5729 * t26392 - 0.18292914397043087775e-1 * t21867 - 0.11433071498151929859e-2 * t21870 + 0.34299214494455789578e-2 * t21874 + 0.17149607247227894789e-2 * t21877 + 0.91464571985215438873e-2 * t7707 * t9311 - 0.91464571985215438873e-2 * t7707 * t9316 - 0.13719685797782315831e-1 * t21604 * t9321;
    (t26387, t26411)
}
