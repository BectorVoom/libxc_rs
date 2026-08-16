//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1426/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1426<F: Float>(t3092: F, t55922: F, t3087: F, t26989: F, t55917: F, t894: F, t27129: F, t1111: F, t1133: F, t123: F, t1503: F, t1509: F, t1520: F, t15311: F, t15327: F, t15355: F, t17898: F, t27061: F, t27152: F, t322: F, t4310: F, t4570: F, t46851: F, t46853: F, t46886: F, t46902: F, t5314: F, t5337: F, t54252: F, t54308: F, t54389: F, t54392: F, t54394: F, t54596: F, t54600: F, t8966: F, t8973: F) -> (F, F, F, F, F) {
    let t59532 = t3092 * t55922;
    let t59536 = t3087 * t55922;
    let t59558 = t894 * t26989 * t55917;
    let t59568 = t894 * t27129 * t55917;
    let t59575 = -t1111 * t322 * t59532 / F::cast_from(48.0_f64) + t1111 * t322 * t59536 / F::cast_from(72.0_f64) + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t4310 * t17898 - F::cast_from(154.0_f64) / F::cast_from(243.0_f64) * t54252 * t1503 - F::cast_from(0.73258227843678641351e2_f64) * t8973 * t15311 * t27152 * t4570 * t123 + F::cast_from(0.36629113921839320675e2_f64) * t8966 * t15311 * t54308 - F::cast_from(0.40246118008281286364e-2_f64) * t46851 + F::cast_from(0.25757515525300023273e-1_f64) * t46853 + F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t46886 + F::cast_from(0.71947308084538596198e1_f64) * t15355 * t5314 + F::cast_from(0.10866451862235947318e0_f64) * t1133 * t59558 - F::cast_from(0.3517423950799664703e2_f64) * t54596 * t1509 - F::cast_from(0.73408919247105066328e0_f64) * t15327 * t5337 - F::cast_from(0.1794440248262568288e1_f64) * t54600 * t1520 - F::cast_from(0.96590683219875087274e-1_f64) * t1133 * t59568 + F::cast_from(0.48295341609937543636e-2_f64) * t46902 - t54389 / F::cast_from(36.0_f64) - F::cast_from(0.48295341609937543636e-1_f64) * t54392 - F::cast_from(0.15146801702008125515e1_f64) * t54394 - t27061;
    (t59532, t59536, t59558, t59568, t59575)
}
