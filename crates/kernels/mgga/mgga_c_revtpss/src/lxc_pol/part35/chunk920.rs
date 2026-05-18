//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 920/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk920<F: Float>(t23278: F, t23310: F, t23339: F, t23357: F, t10645: F, t10651: F, t10952: F, t14512: F, t14525: F, t14533: F, t14558: F, t14564: F, t1559: F, t18690: F, t18699: F, t213: F, t23160: F, t23168: F, t23172: F, t23177: F, t23245: F, t234: F, t2811: F, t4494: F, t4504: F, t4514: F, t4526: F, t5978: F, t6017: F, t820: F, t879: F) -> (F, F) {
    let t23359 = t23278 + t23310 + t23339 + t23357;
    let t23363 = -F::new(0.19756347548806534796e1) * t4514 * t18699 * t1559 + F::new(0.19514881078765566038e-2) * t14512 + F::new(0.39512695097613069591e1) * t4504 * t4494 * t23160 - F::new(0.34697458558045176417e-2) * t14525 - F::new(0.21951497276451705329e-1) * t14533 - F::new(0.16463622957338778996e-1) * t18690 - F::new(0.39512695097613069591e1) * t820 * t10952 * t23168 + F::new(0.39512695097613069591e1) * t820 * t2811 * t23172 - F::new(0.19514881078765566038e-2) * t14558 - F::new(0.65854491829355115987e0) * t820 * t879 * t23177 - F::new(0.19756347548806534796e1) * t820 * t4526 * t5978 + F::new(0.39029762157531132076e-1) * t14564 - F::new(0.65854491829355115987e0) * t820 * t879 * t23245 - t10645 + t10651 - F::new(0.19756347548806534796e1) * t820 * t4526 * t6017 + F::new(0.65854491829355115987e0) * t213 * t234 * t23359;
    (t23359, t23363)
}
