//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1188/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1188(t11311: f64, t11313: f64, t5987: f64, t11312: f64, t3064: f64, t3949: f64, t1036: f64, t13790: f64, t1649: f64, t19677: f64, t33273: f64, t11387: f64, t5248: f64, t5553: f64) -> (f64, f64, f64, f64, f64) {
    let t34723 = t5987 * t11311 * t11313;
    let t34726 = t11312 * t3064 * t3949;
    let t34729 = t11312 * t1036 * t13790;
    let t34732 = t19677 * t33273 * t1649;
    let t34735 = t5553 * t11387 * t5248;
    (t34723, t34726, t34729, t34732, t34735)
}
