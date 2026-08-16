//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3150/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3150(t56228: f64, t43858: f64, t43865: f64, t43883: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t56212: f64, t56214: f64, t56216: f64, t56221: f64, t56226: f64, t56230: f64, t56234: f64, t56236: f64, t56248: f64, t56252: f64, t56256: f64) -> f64 {
    let t58090 = 4.0_f64 / 9.0_f64 * t56228;
    let t58105 = 2.0_f64 / 9.0_f64 * t56212 + 4.0_f64 / 3.0_f64 * t56214 - 10.0_f64 / 27.0_f64 * t56216 + 10.0_f64 / 9.0_f64 * t56221 + 2.0_f64 * t56226 + t58090 - t56230 / 3.0_f64 + t56234 / 3.0_f64 - 28.0_f64 / 81.0_f64 * t56236 - 10.0_f64 / 81.0_f64 * t43858 - 8.0_f64 / 27.0_f64 * t43865 + 4.0_f64 / 9.0_f64 * t43883 - 28.0_f64 / 27.0_f64 * t43888 + 4.0_f64 / 9.0_f64 * t43890 + 8.0_f64 / 9.0_f64 * t43892 - 2.0_f64 / 3.0_f64 * t43894 - t43896 / 9.0_f64 + 10.0_f64 / 9.0_f64 * t56248 + 6.0_f64 * t56252 - 4.0_f64 * t56256;
    t58105
}
