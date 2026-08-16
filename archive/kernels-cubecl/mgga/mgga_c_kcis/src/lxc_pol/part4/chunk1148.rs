//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1148/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1148<F: Float>(t2861: F, t4986: F, t1773: F, t3316: F, t1131: F, t1096: F, t1092: F, t1010: F, t10450: F, t10452: F, t10472: F, t10473: F, t14095: F, t14100: F, t14103: F, t14104: F, t14108: F, t14113: F, t14377: F, t14384: F, t14388: F, t14390: F, t14564: F, t14568: F, t14570: F, t14574: F, t979: F) -> (F, F, F, F) {
    let t14576 = t2861 * t4986;
    let t14577 = F::cast_from(0.22109259259259259258e-2_f64) * t14576;
    let t14578 = t1773 * t3316;
    let t14579 = t1131 * t14578;
    let t14580 = t1096 * t14579;
    let t14581 = t1092 * t14580;
    let t14583 = F::cast_from(0.99491666666666666664e-2_f64) * t14095 + F::cast_from(0.49745833333333333332e-2_f64) * t14100 + t14103 - F::cast_from(0.22109259259259259258e-2_f64) * t14104 + F::cast_from(0.29479012345679012345e-2_f64) * t14108 - F::cast_from(0.55273148148148148147e-3_f64) * t14113 - F::cast_from(0.24872916666666666666e-2_f64) * t14377 - F::cast_from(0.11054629629629629629e-2_f64) * t10450 + F::cast_from(0.16581944444444444444e-2_f64) * t10452 + F::cast_from(0.66327777777777777776e-2_f64) * t14384 - F::cast_from(0.55273148148148148146e-2_f64) * t14388 - F::cast_from(0.3684876543209876543e-3_f64) * t14390 - F::cast_from(0.66725e-1_f64) * t979 * t14564 + t14568 + t10472 + F::cast_from(0.29479012345679012345e-2_f64) * t10473 - F::cast_from(0.13345e0_f64) * t14570 * t1010 + F::cast_from(0.27636574074074074073e-2_f64) * t14574 - t14577 - F::cast_from(0.16581944444444444444e-2_f64) * t14581;
    (t14576, t14578, t14581, t14583)
}
