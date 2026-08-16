//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 870/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk870(t12885: f64, t13064: f64, t3725: f64, t12916: f64, t12919: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12935: f64, t12937: f64, t12939: f64, t12943: f64, t12946: f64, t12948: f64, t12954: f64) -> (f64, f64) {
    let t13066 = t13064 * t12885 * t3725;
    let t13083 = -0.3883875e1_f64 * t12916 + 0.247573125e0_f64 * t12919 - 0.33547222222222222222e0_f64 * t12922 - 0.301925e0_f64 * t12927 - 0.40256666666666666668e0_f64 * t12929 + 0.30192500000000000001e0_f64 * t12931 + 0.20128333333333333333e0_f64 * t12933 - 0.27595e0_f64 * t12935 + 0.16557e0_f64 * t12937 + 0.5519e-1_f64 * t12939 - 0.36793333333333333333e-1_f64 * t12943 - 0.82785e-1_f64 * t12946 - 0.60385000000000000001e0_f64 * t12948 + 0.12077e1_f64 * t12954;
    (t13066, t13083)
}
