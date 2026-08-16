//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 935/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk935(t20535: f64, t34688: f64, t9537: f64, t26796: f64, t9282: f64, t20671: f64, t31037: f64, t35101: f64, t1457: f64, t1572: f64, t41865: f64, t12904: f64, t4614: f64, t574: f64) -> (f64, f64, f64, f64, f64) {
    let t42066 = t20535 * t34688 * t9537;
    let t42067 = 0.11502877786176224903e1_f64 * t42066;
    let t42069 = 0.47667319935800568892e0_f64 * t26796 * t9282;
    let t42071 = t31037 * t20671 * t35101;
    let t42072 = 0.25561950635947166451e0_f64 * t42071;
    let t42074 = t1572 * t1457 * t41865;
    let t42077 = t574 * t4614 * t12904;
    (t42067, t42069, t42072, t42074, t42077)
}
