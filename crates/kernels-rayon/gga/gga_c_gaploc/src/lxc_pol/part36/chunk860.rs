//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 860/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk860(t18313: f64, t18372: f64, t41596: f64, t590: f64, t20535: f64, t34688: f64, t9537: f64, t26796: f64, t9282: f64, t20671: f64, t31037: f64, t35101: f64) -> (f64, f64, f64, f64) {
    let t42064 = 0.61348681526273199482e1_f64 * t18372 * t18313 * t41596 * t590;
    let t42066 = t20535 * t34688 * t9537;
    let t42067 = 0.11502877786176224903e1_f64 * t42066;
    let t42069 = 0.47667319935800568892e0_f64 * t26796 * t9282;
    let t42071 = t31037 * t20671 * t35101;
    (t42064, t42067, t42069, t42071)
}
