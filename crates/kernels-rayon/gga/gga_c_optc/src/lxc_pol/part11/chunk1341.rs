//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1341/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1341(t57215: f64, t57217: f64, t57219: f64, t57222: f64, t57225: f64, t57228: f64, t57233: f64, t57236: f64, t57238: f64, t57240: f64, t57244: f64, t1000: f64, t13912: f64, t1415: f64, t24392: f64, t2549: f64, t3980: f64, t43260: f64, t5076: f64, t52200: f64, t52241: f64, t52245: f64, t57010: f64, t57014: f64, t57018: f64, t57032: f64, t57039: f64, t57246: f64, t57248: f64, t7254: f64, t914: f64, t999: f64) -> (f64, f64) {
    let t58198 = t57215 + t57217 + t57219 - t57222 + t57225 + t57228 - t57233 - t57236 - t57238 - t57240 - t57244;
    let t58226 = -0.10337952573961372198e-1_f64 * t3980 * t52200 * t1415 - 4.0_f64 / 9.0_f64 * t43260 - 4.0_f64 * t999 * t914 * t1000 * t57010 - 2.0_f64 * t13912 * t5076 + 2.0_f64 / 3.0_f64 * t999 * t914 * t2549 * t57018 - t999 * t914 * t1000 * t57014 - t57246 + 8.0_f64 / 9.0_f64 * t52241 + 2.0_f64 / 3.0_f64 * t52245 - 56.0_f64 / 9.0_f64 * t999 * t914 * t7254 * t57032 + 140.0_f64 / 81.0_f64 * t999 * t914 * t24392 * t57039 - t57248;
    (t58198, t58226)
}
