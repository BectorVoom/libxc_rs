//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 841/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk841(t29740: f64, t7362: f64, t6260: f64, t7376: f64, t7375: f64, t1244: f64, t2121: f64, t2149: f64, t24773: f64, t24849: f64, t27406: f64, t27451: f64, t27556: f64, t29678: f64, t29702: f64, t29705: f64, t29709: f64, t29712: f64, t29716: f64, t29720: f64, t29723: f64, t29727: f64, t29736: f64, t3610: f64, t3624: f64, t5064: f64, t7283: f64, t7373: f64, t8070: f64, t8083: f64) -> f64 {
    let t29741 = t7362 * t29740;
    let t29744 = t6260 * t7376;
    let t29745 = t7375 * t29744;
    let t29748 = -0.18277045187202515961e-2_f64 * t27451 - t24773 - 0.82246703342411321825e-2_f64 * t7283 * t29702 - 0.82246703342411321825e-2_f64 * t7283 * t29705 - t3624 * t29709 + t1244 * t29712 + 2.0_f64 * t5064 * t8083 - 0.16449340668482264365e-1_f64 * t7373 * t29716 + 2.0_f64 * t1244 * t29720 + 2.0_f64 * t3610 * t29723 + 0.82246703342411321825e-2_f64 * t2121 * t29727 + 0.80418998823691070228e-1_f64 * t29678 * t2149 + 0.43864908449286038306e-1_f64 * t27406 * t8070 - 0.54831135561607547884e-2_f64 * t24849 * t29736 + 0.54831135561607547884e-2_f64 * t27556 - 0.54831135561607547884e-2_f64 * t7283 * t29741 + 0.82246703342411321825e-2_f64 * t7373 * t29745;
    t29748
}
