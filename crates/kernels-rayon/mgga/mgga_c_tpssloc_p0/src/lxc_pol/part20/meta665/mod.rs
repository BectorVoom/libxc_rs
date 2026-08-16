//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2492;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2493;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2494;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2495;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2496;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2497;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2498;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2499;
use chunk8::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta665(t1409: f64, t43791: f64, t9288: f64, t11145: f64, t123: f64, t2394: f64, t4725: f64, t14727: f64, t690: f64, t43763: f64, t43809: f64, t12606: f64, t3247: f64, t607: f64, t1088: f64, t50865: f64, t50869: f64, t50873: f64, t50903: f64, t50905: f64, t50907: f64, t50912: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50915, t50917) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2492(t1409, t43791, t9288, t11145, t123);
        let t50919 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2493(t2394, t4725);
        let t50921 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2494(t14727, t690);
        let (t50924, t50926) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2495(t1409, t43763, t9288, t123, t43809);
        let (t50929, t50931) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2496(t12606, t3247, t607, t1088, t123);
        let t50934 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2497(t1088, t123, t50865);
        let t50937 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2498(t1088, t123, t50869);
        let t50940 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2499(t1088, t123, t50873);
        let t50942 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2500(t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934, t50937, t50940);
    (t50915, t50917, t50919, t50921, t50924, t50926, t50929, t50931, t50934, t50937, t50940, t50942)
}
