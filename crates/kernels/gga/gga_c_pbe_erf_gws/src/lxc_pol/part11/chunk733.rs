//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 733/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk733<F: Float>(t10933: F, t10973: F, t10993: F, t7811: F, t12476: F, t1821: F, t587: F, t12468: F, t2559: F, t3421: F, t995: F, t1820: F, t1017: F, t5543: F, t12774: F, t12775: F, t12777: F, t12781: F, t12785: F, t12786: F, t12787: F, t12788: F, t12789: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12790 = 16.0 / 45.0 * t10933;
    let t12791 = 8.0 / 15.0 * t10973;
    let t12792 = 16.0 / 45.0 * t10993;
    let t12793 = 4.0 / 45.0 * t7811;
    let t12794 = t1821 * t12476;
    let t12796 = 8.0 / 15.0 * t587 * t12794;
    let t12797 = t2559 * t12468;
    let t12799 = 4.0 / 9.0 * t587 * t12797;
    let t12800 = t3421 * t995;
    let t12801 = t2559 * t12800;
    let t12803 = 8.0 / 9.0 * t1820 * t12801;
    let t12804 = t3421 * t1017;
    let t12805 = t5543 * t12804;
    let t12807 = 4.0 / 9.0 * t587 * t12805;
    let t12808 = t12774 + t12775 + t12777 + t12781 - t12785 - t12786 + t12787 + t12788 - t12789 + t12790 + t12791 - t12792 - t12793 - t12796 + t12799 + t12803 - t12807;
    (t12790, t12791, t12792, t12793, t12794, t12796, t12797, t12799, t12800, t12801, t12803, t12804, t12805, t12807, t12808)
}
