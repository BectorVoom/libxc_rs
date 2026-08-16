//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 829/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk829(t33049: f64, t33130: f64, t33174: f64, t33219: f64, t160: f64, t33119: f64, t1349: f64, t149: f64, t32743: f64, t32750: f64, t32872: f64, t32876: f64, t32881: f64, t32993: f64, t32995: f64, t32998: f64, t33002: f64, t33045: f64, t33086: f64, t33091: f64, t33179: f64, t5766: f64, t5772: f64, t5781: f64, t5849: f64, t7309: f64, t7315: f64, t7342: f64) -> (f64, f64, f64) {
    let t33221 = t33049 + t33130 + t33174 + t33219;
    let t33227 = t33119 * t160;
    let t33229 = -t1349 * t32743 / 3.0_f64 + t5766 * t7342 / 6.0_f64 + t32750 + t1349 * t32872 / 6.0_f64 + t1349 * t32876 / 6.0_f64 - t5772 * t32881 / 9.0_f64 - t5766 * t7315 / 3.0_f64 + t7309 * t5849 / 6.0_f64 - 2.0_f64 * t32993 + 4.0_f64 * t32995 + t1349 * t32998 - 2.0_f64 / 3.0_f64 * t1349 * t33002 - t7309 * t5781 / 3.0_f64 - t149 * t33221 + 8.0_f64 * t33179 + 4.0_f64 * t33045 - 12.0_f64 * t33086 + 8.0_f64 * t33091 + 2.0_f64 * t33227;
    (t33221, t33227, t33229)
}
