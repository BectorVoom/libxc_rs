//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 722/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk722<F: Float>(t10473: F, t2478: F, t6576: F, t4130: F, t41596: F, t4781: F, t590: F, t34688: F, t9272: F, t9273: F, t18313: F, t31119: F, t3394: F, t10495: F, t1424: F, t2299: F, t544: F) -> (F, F, F, F, F) {
    let t41649 = t6576 * t10473 * t2478;
    let t41650 = 0.76685851907841499353e0 * t41649;
    let t41654 = 0.13803453343411469884e2 * t4781 * t4130 * t41596 * t590;
    let t41656 = t9272 * t34688 * t9273;
    let t41657 = 0.10352590007558602413e2 * t41656;
    let t41660 = t31119 * t18313 * t3394 * t9273;
    let t41661 = 0.46011511144704899612e1 * t41660;
    let t41664 = t544 * t2299 * t10495 * t1424;
    (t41650, t41654, t41657, t41661, t41664)
}
