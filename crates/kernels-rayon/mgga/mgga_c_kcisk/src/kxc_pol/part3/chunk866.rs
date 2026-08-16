//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 866/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk866(t12970: f64, t13009: f64, t12959: f64, t12962: f64, t12965: f64, t12967: f64, t12971: f64, t12985: f64, t12989: f64, t12993: f64, t12995: f64, t12999: f64, t13000: f64, t13002: f64, t13005: f64) -> (f64, f64) {
    let t13010 = t13009 * t12970;
    let t13014 = -0.17938e1_f64 * t12959 + 0.16431333333333333333e0_f64 * t12962 - 0.49293999999999999999e0_f64 * t12965 - 0.32862666666666666666e0_f64 * t12967 - 0.76790625e-1_f64 * t12971 + 0.1898925e1_f64 * t12993 + 0.3071625e0_f64 * t12995 - t12999 - t13000 - 0.82156666666666666668e-1_f64 * t13002 + 0.49293999999999999999e0_f64 * t13005 + 0.142419375e1_f64 * t13010 - 0.59793333333333333333e0_f64 * t12985 + 0.17938e1_f64 * t12989;
    (t13010, t13014)
}
