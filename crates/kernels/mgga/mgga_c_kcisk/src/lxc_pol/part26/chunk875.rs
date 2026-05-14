//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 875/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk875<F: Float>(t3739: F, t6003: F, t5882: F, t2214: F, t3805: F, t5969: F, t5993: F, t5998: F, t3748: F, t5607: F, t1450: F, t3483: F, t5988: F, t5985: F, t5887: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19759 = t3739 * t6003;
    let t19760 = 0.22109259259259259258e-2 * t19759;
    let t19761 = t3739 * t5882;
    let t19762 = 0.22109259259259259258e-2 * t19761;
    let t19788 = t3805 * t2214;
    let t19790 = t3739 * t5969;
    let t19791 = 0.33163888888888888888e-2 * t19790;
    let t19806 = t3739 * t5993;
    let t19807 = 0.66327777777777777776e-2 * t19806;
    let t19808 = t3739 * t5998;
    let t19810 = t3748 * t5607;
    let t19813 = t3483 * t1450;
    let t19832 = t3739 * t5988;
    let t19833 = 0.33163888888888888888e-2 * t19832;
    let t19837 = t3739 * t5985;
    let t19846 = t3739 * t5887;
    (t19759, t19760, t19761, t19762, t19788, t19790, t19791, t19806, t19807, t19808, t19810, t19813, t19832, t19833, t19837, t19846)
}
