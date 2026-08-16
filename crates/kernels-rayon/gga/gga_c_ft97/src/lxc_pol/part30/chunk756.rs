//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 756/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk756(t263: f64, t7484: f64, t684: f64, t2354: f64, t5994: f64, t7150: f64, t1439: f64, t1774: f64, t7426: f64, t1425: f64, t666: f64, t461: f64, t6144: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33535 = t7484 * t263;
    let t33536 = t33535 * t684;
    let t33537 = t2354 * t33536;
    let t33540 = t5994 * t7150;
    let t33543 = t1774 * t1439;
    let t33545 = t7426 * t33543 / 18.0_f64;
    let t33546 = t1425 * t684;
    let t33547 = t666 * t33546;
    let t33552 = t461 * t6144;
    (t33535, t33537, t33540, t33543, t33545, t33546, t33547, t33552)
}
