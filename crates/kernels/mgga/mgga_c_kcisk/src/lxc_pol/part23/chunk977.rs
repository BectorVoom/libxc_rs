//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 977/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk977<F: Float>(t19802: F, t3785: F, t1411: F, t3739: F, t5993: F, t5998: F, t3748: F, t5607: F, t1450: F, t3483: F, t3486: F, t5675: F, t3482: F, t5670: F, t5633: F, t2075: F, t3742: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19803 = t3785 * t19802;
    let t19804 = t1411 * t19803;
    let t19806 = t3739 * t5993;
    let t19807 = 0.66327777777777777776e-2 * t19806;
    let t19808 = t3739 * t5998;
    let t19810 = t3748 * t5607;
    let t19813 = t3483 * t1450;
    let t19814 = t5675 * t3486;
    let t19815 = t19813 * t19814;
    let t19816 = t3482 * t19815;
    let t19818 = t5670 * t3486;
    let t19819 = t19813 * t19818;
    let t19820 = t5633 * t19819;
    let t19822 = t2075 * t3742;
    (t19804, t19806, t19807, t19808, t19810, t19814, t19816, t19818, t19820, t19822)
}
