//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 587/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk587(t1656: f64, t428: f64, t374: f64, t1751: f64, t384: f64, t401: f64, t1685: f64, t1594: f64, t7929: f64, t1603: f64, t1624: f64, t1664: f64, t1669: f64, t1670: f64, t1686: f64, t1710: f64, t3076: f64, t409: f64, t429: f64, t5517: f64, t64: f64, t8070: f64, t8139: f64, t8147: f64, t8154: f64, t8157: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8161 = t1656 * t428;
    let t8162 = t374 * t8161;
    let t8165 = t384 * t1751;
    let t8166 = t374 * t8165;
    let t8169 = t1656 * t401;
    let t8170 = t374 * t8169;
    let t8173 = t384 * t1685;
    let t8174 = t374 * t8173;
    let t8177 = t1594 * t7929;
    let t8180 = 6.0_f64 * t5517 * t1686 - 6.0_f64 * t1664 * t429 + 2.0_f64 * t8070 - t64 * t409 * t8139 + 6.0_f64 * t3076 * t1710 * t428 * t1751 - 6.0_f64 * t1669 * t8147 - 6.0_f64 * t1669 * t1670 * t1751 + 0.10261957230907473486e-6_f64 * t3076 * t8154 * t8157 + 0.34882351419656688e-1_f64 * t1624 * t8162 + 0.34882351419656688e-1_f64 * t1624 * t8166 - 0.69764702839313376e-1_f64 * t1603 * t8170 - 0.69764702839313376e-1_f64 * t1603 * t8174 + 0.11619434043764639964e-2_f64 * t1603 * t8177;
    (t8161, t8165, t8169, t8173, t8174, t8180)
}
