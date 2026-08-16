//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 755/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk755(t33517: f64, t33530: f64, t258: f64, t1403: f64, t33245: f64, t33248: f64, t33255: f64, t33259: f64, t33264: f64, t33269: f64, t33272: f64, t33275: f64, t33279: f64, t33490: f64, t33496: f64, t33499: f64, t33504: f64, t5996: f64, t6002: f64, t6005: f64, t6011: f64, t6064: f64, t6068: f64, t7437: f64, t7491: f64) -> (f64, f64, f64) {
    let t33531 = t33517 + t33530;
    let t33532 = t33531 * t258;
    let t33534 = t1403 * t33245 - 2.0_f64 / 3.0_f64 * t1403 * t33248 - t7437 * t6011 / 3.0_f64 - t1403 * t33255 / 3.0_f64 + t1403 * t33259 / 3.0_f64 + t7437 * t6068 / 6.0_f64 - 4.0_f64 * t33264 + t5996 * t7491 / 3.0_f64 + t1403 * t33269 / 3.0_f64 - 4.0_f64 * t33272 - 2.0_f64 * t33275 - 2.0_f64 / 3.0_f64 * t1403 * t33279 - 2.0_f64 * t33490 + t7437 * t6064 / 6.0_f64 + t6002 * t33496 / 9.0_f64 - t33499 * t6005 / 18.0_f64 - t6002 * t33504 / 9.0_f64 + 2.0_f64 * t33532;
    (t33531, t33532, t33534)
}
