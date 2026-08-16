//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 578/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk578(t1328: f64, t8059: f64, t2173: f64, t3924: f64, t1220: f64, t2174: f64, t3807: f64, t3930: f64, t412: f64, t5880: f64, t5972: f64, t5979: f64, t6221: f64, t7828: f64, t7834: f64, t7837: f64, t7840: f64, t7909: f64) -> (f64, f64, f64, f64) {
    let t8060 = t8059 * t1328;
    let t8063 = t2173 * t2173;
    let t8064 = t8063 * t3924;
    let t8071 = t7828 * t412 + 0.16581944444444444444e-2_f64 * t7834 - 0.49745833333333333332e-2_f64 * t7837 + 0.33163888888888888888e-2_f64 * t7840 - 0.24872916666666666666e-2_f64 * t7909 - t3807 + 0.33163888888888888888e-2_f64 * t5880 - 0.193e0_f64 * t1220 * t8060 + 0.74498e-1_f64 * t3930 * t8064 - 0.33163888888888888888e-2_f64 * t5972 + 0.22109259259259259258e-2_f64 * t5979 - 0.386e0_f64 * t6221 * t2174;
    (t8060, t8063, t8064, t8071)
}
