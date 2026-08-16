//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1148/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1148(t2861: f64, t4986: f64, t1773: f64, t3316: f64, t1131: f64, t1096: f64, t1092: f64, t1010: f64, t10450: f64, t10452: f64, t10472: f64, t10473: f64, t14095: f64, t14100: f64, t14103: f64, t14104: f64, t14108: f64, t14113: f64, t14377: f64, t14384: f64, t14388: f64, t14390: f64, t14564: f64, t14568: f64, t14570: f64, t14574: f64, t979: f64) -> (f64, f64, f64, f64) {
    let t14576 = t2861 * t4986;
    let t14577 = 0.22109259259259259258e-2_f64 * t14576;
    let t14578 = t1773 * t3316;
    let t14579 = t1131 * t14578;
    let t14580 = t1096 * t14579;
    let t14581 = t1092 * t14580;
    let t14583 = 0.99491666666666666664e-2_f64 * t14095 + 0.49745833333333333332e-2_f64 * t14100 + t14103 - 0.22109259259259259258e-2_f64 * t14104 + 0.29479012345679012345e-2_f64 * t14108 - 0.55273148148148148147e-3_f64 * t14113 - 0.24872916666666666666e-2_f64 * t14377 - 0.11054629629629629629e-2_f64 * t10450 + 0.16581944444444444444e-2_f64 * t10452 + 0.66327777777777777776e-2_f64 * t14384 - 0.55273148148148148146e-2_f64 * t14388 - 0.3684876543209876543e-3_f64 * t14390 - 0.66725e-1_f64 * t979 * t14564 + t14568 + t10472 + 0.29479012345679012345e-2_f64 * t10473 - 0.13345e0_f64 * t14570 * t1010 + 0.27636574074074074073e-2_f64 * t14574 - t14577 - 0.16581944444444444444e-2_f64 * t14581;
    (t14576, t14578, t14581, t14583)
}
