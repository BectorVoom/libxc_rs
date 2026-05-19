//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 587/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk587<F: Float>(t1656: F, t428: F, t374: F, t1751: F, t384: F, t401: F, t1685: F, t1594: F, t7929: F, t1603: F, t1624: F, t1664: F, t1669: F, t1670: F, t1686: F, t1710: F, t3076: F, t409: F, t429: F, t5517: F, t64: F, t8070: F, t8139: F, t8147: F, t8154: F, t8157: F) -> (F, F, F, F, F, F) {
    let t8161 = t1656 * t428;
    let t8162 = t374 * t8161;
    let t8165 = t384 * t1751;
    let t8166 = t374 * t8165;
    let t8169 = t1656 * t401;
    let t8170 = t374 * t8169;
    let t8173 = t384 * t1685;
    let t8174 = t374 * t8173;
    let t8177 = t1594 * t7929;
    let t8180 = F::new(6.0) * t5517 * t1686 - F::new(6.0) * t1664 * t429 + F::new(2.0) * t8070 - t64 * t409 * t8139 + F::new(6.0) * t3076 * t1710 * t428 * t1751 - F::new(6.0) * t1669 * t8147 - F::new(6.0) * t1669 * t1670 * t1751 + F::cast_from(0.10261957230907473486e-6_f64) * t3076 * t8154 * t8157 + F::cast_from(0.34882351419656688e-1_f64) * t1624 * t8162 + F::cast_from(0.34882351419656688e-1_f64) * t1624 * t8166 - F::cast_from(0.69764702839313376e-1_f64) * t1603 * t8170 - F::cast_from(0.69764702839313376e-1_f64) * t1603 * t8174 + F::cast_from(0.11619434043764639964e-2_f64) * t1603 * t8177;
    (t8161, t8165, t8169, t8173, t8174, t8180)
}
