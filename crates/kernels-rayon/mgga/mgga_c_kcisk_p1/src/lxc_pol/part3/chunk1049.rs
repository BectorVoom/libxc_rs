//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1049/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1049(t12489: f64, t12491: f64, t12493: f64, t12524: f64, t12526: f64, t12528: f64, t5680: f64, t5744: f64, t1078: f64, t15579: f64, t12610: f64, t47: f64) -> (f64, f64, f64) {
    let t15591 = -0.47063e1_f64 * t12489 + 0.31375333333333333334e1_f64 * t12491 - 0.36604555555555555556e1_f64 * t12493 - 0.16068111111111111111e1_f64 * t5680 + 0.28051666666666666666e0_f64 * t12524 - 0.56103333333333333332e0_f64 * t12526 - 0.6545388888888888889e0_f64 * t12528 - 0.46308888888888888888e0_f64 * t5744;
    let t15592 = t15591 * t1078;
    let t15595 = t15579 * t1078;
    let t15598 = t47 * t12610;
    (t15592, t15595, t15598)
}
