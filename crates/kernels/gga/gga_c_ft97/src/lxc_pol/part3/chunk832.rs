//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 832/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk832<F: Float>(t113: F, t4375: F, t1274: F, t332: F, t992: F, t4380: F, t1578: F, t505: F, t5479: F, t4376: F, t4635: F, t910: F, t1091: F, t2923: F, t4370: F, t2253: F, t5470: F) -> (F, F, F, F, F, F, F, F) {
    let t18794 = t113 * t4375;
    let t18795 = t1274 * t18794;
    let t18798 = t332 * t992;
    let t18799 = t4380 * t18798;
    let t18802 = t1274 * t1578;
    let t18804 = t5479 * t505;
    let t18809 = t4376 * t992;
    let t18812 = t910 * t4635;
    let t18820 = t2923 * t1091 * t4370;
    let t18823 = t2253 * t5470;
    (t18795, t18799, t18802, t18804, t18809, t18812, t18820, t18823)
}
