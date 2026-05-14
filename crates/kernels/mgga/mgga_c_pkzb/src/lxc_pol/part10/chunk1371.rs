//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1371/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1371<F: Float>(t8202: F, t8219: F, t22727: F, t8206: F, t2234: F, t2240: F, t3740: F, t2198: F, t3769: F, t6199: F, t1185: F, t2197: F, t8003: F, t6142: F, t18617: F, t9867: F) -> (F, F, F, F, F, F, F) {
    let t27406 = 0.32163958997385070134e2 * t8219 * t8202;
    let t27408 = 0.1034520258385468006e4 * t22727 * t8206;
    let t27411 = 6.0 * t2240 * t3740 * t2234;
    let t27414 = 0.57895126195293126241e3 * t6199 * t3769 * t2198;
    let t27417 = 4.0 * t2197 * t1185 * t8003;
    let t27420 = 0.96491876992155210402e2 * t6142 * t3769 * t2234;
    let t27423 = 0.62071215503128080361e4 * t18617 * t9867 * t2198;
    (t27406, t27408, t27411, t27414, t27417, t27420, t27423)
}
