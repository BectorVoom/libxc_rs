//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 830/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk830(t41738: f64, t6716: f64, t6717: f64, t12875: f64, t18651: f64, t40103: f64, t10526: f64, t20471: f64, t41712: f64, t41713: f64, t41714: f64, t41715: f64, t41716: f64, t41717: f64, t41718: f64, t41719: f64, t41721: f64, t41724: f64, t41729: f64, t41731: f64, t41734: f64, t41735: f64, t41736: f64, t41737: f64) -> f64 {
    let t41741 = 0.62115540045351614476e2_f64 * t6716 * t6717 * t41738;
    let t41743 = 0.27606906686822939767e2_f64 * t18651 * t12875;
    let t41744 = 0.23005755572352449806e1_f64 * t40103;
    let t41747 = 0.21450293971110256001e1_f64 * t20471 * t10526 * t41738;
    let t41748 = -t41712 + t41713 - t41714 - t41715 + t41716 + t41717 + t41718 + t41719 + t41721 - 0.71500979903700853338e0_f64 * t41724 - t41729 + 0.59584149919750711116e-1_f64 * t41731 - t41734 - t41735 - t41736 + t41737 + t41741 + t41743 - t41744 + t41747;
    t41748
}
