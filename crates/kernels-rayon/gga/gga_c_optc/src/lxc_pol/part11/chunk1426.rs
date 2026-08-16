//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1426/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1426(t3092: f64, t55922: f64, t3087: f64, t26989: f64, t55917: f64, t894: f64, t27129: f64, t1111: f64, t1133: f64, t123: f64, t1503: f64, t1509: f64, t1520: f64, t15311: f64, t15327: f64, t15355: f64, t17898: f64, t27061: f64, t27152: f64, t322: f64, t4310: f64, t4570: f64, t46851: f64, t46853: f64, t46886: f64, t46902: f64, t5314: f64, t5337: f64, t54252: f64, t54308: f64, t54389: f64, t54392: f64, t54394: f64, t54596: f64, t54600: f64, t8966: f64, t8973: f64) -> (f64, f64, f64, f64, f64) {
    let t59532 = t3092 * t55922;
    let t59536 = t3087 * t55922;
    let t59558 = t894 * t26989 * t55917;
    let t59568 = t894 * t27129 * t55917;
    let t59575 = -t1111 * t322 * t59532 / 48.0_f64 + t1111 * t322 * t59536 / 72.0_f64 + 8.0_f64 / 27.0_f64 * t4310 * t17898 - 154.0_f64 / 243.0_f64 * t54252 * t1503 - 0.73258227843678641351e2_f64 * t8973 * t15311 * t27152 * t4570 * t123 + 0.36629113921839320675e2_f64 * t8966 * t15311 * t54308 - 0.40246118008281286364e-2_f64 * t46851 + 0.25757515525300023273e-1_f64 * t46853 + 2.0_f64 / 81.0_f64 * t46886 + 0.71947308084538596198e1_f64 * t15355 * t5314 + 0.10866451862235947318e0_f64 * t1133 * t59558 - 0.3517423950799664703e2_f64 * t54596 * t1509 - 0.73408919247105066328e0_f64 * t15327 * t5337 - 0.1794440248262568288e1_f64 * t54600 * t1520 - 0.96590683219875087274e-1_f64 * t1133 * t59568 + 0.48295341609937543636e-2_f64 * t46902 - t54389 / 36.0_f64 - 0.48295341609937543636e-1_f64 * t54392 - 0.15146801702008125515e1_f64 * t54394 - t27061;
    (t59532, t59536, t59558, t59568, t59575)
}
