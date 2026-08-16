//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1170/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1170(t10645: f64, t10651: f64, t10952: f64, t14512: f64, t14525: f64, t14533: f64, t14558: f64, t14564: f64, t1559: f64, t18690: f64, t18699: f64, t213: f64, t23160: f64, t23168: f64, t23172: f64, t23177: f64, t23245: f64, t23359: f64, t234: f64, t2811: f64, t4494: f64, t4504: f64, t4514: f64, t4526: f64, t5978: f64, t6017: f64, t820: f64, t879: f64) -> f64 {
    let t23363 = -0.19756347548806534796e1_f64 * t4514 * t18699 * t1559 + 0.19514881078765566038e-2_f64 * t14512 + 0.39512695097613069591e1_f64 * t4504 * t4494 * t23160 - 0.34697458558045176417e-2_f64 * t14525 - 0.21951497276451705329e-1_f64 * t14533 - 0.16463622957338778996e-1_f64 * t18690 - 0.39512695097613069591e1_f64 * t820 * t10952 * t23168 + 0.39512695097613069591e1_f64 * t820 * t2811 * t23172 - 0.19514881078765566038e-2_f64 * t14558 - 0.65854491829355115987e0_f64 * t820 * t879 * t23177 - 0.19756347548806534796e1_f64 * t820 * t4526 * t5978 + 0.39029762157531132076e-1_f64 * t14564 - 0.65854491829355115987e0_f64 * t820 * t879 * t23245 - t10645 + t10651 - 0.19756347548806534796e1_f64 * t820 * t4526 * t6017 + 0.65854491829355115987e0_f64 * t213 * t234 * t23359;
    t23363
}
