//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1098/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1098(t152673: f64, t24976: f64, t6317: f64, t152678: f64, t99391: f64, t33847: f64, t4162: f64, t44280: f64, t10248: f64, t28755: f64, t3746: f64, t10683: f64, t24980: f64, t28776: f64) -> (f64, f64, f64, f64, f64) {
    let t152727 = t6317 * t24976 * t152673;
    let t152730 = t6317 * t99391 * t152678;
    let t152734 = t6317 * t44280 * t33847 * t4162;
    let t152738 = t28755 * t10248 * t33847 * t3746;
    let t152742 = t24980 * t10683 * t33847 * t28776;
    (t152727, t152730, t152734, t152738, t152742)
}
