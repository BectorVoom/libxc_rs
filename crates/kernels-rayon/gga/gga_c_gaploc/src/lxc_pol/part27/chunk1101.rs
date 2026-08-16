//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1101/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1101(t23492: f64, t959: f64, t23495: f64, t23292: f64, t787: f64, t9824: f64, t107: f64, t408: f64, t2558: f64, t9823: f64, t22909: f64, t9820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28425 = 0.29792074959875355558e-1_f64 * t23492 * t959;
    let t28427 = 0.59584149919750711116e-1_f64 * t23495 * t959;
    let t28435 = t787 * t23292;
    let t28437 = 0.29792074959875355558e-1_f64 * t28435 * t9824;
    let t28438 = t107 * t408;
    let t28439 = t28438 * t2558;
    let t28441 = 0.11916829983950142223e0_f64 * t9823 * t28439;
    let t28443 = 0.29792074959875355558e-1_f64 * t9820 * t22909;
    (t28425, t28427, t28435, t28437, t28438, t28439, t28441, t28443)
}
