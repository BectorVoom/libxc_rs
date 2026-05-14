//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 692/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk692<F: Float>(t10417: F, t1415: F, t7030: F, t12960: F, t31051: F, t10473: F, t2478: F, t6576: F, t34688: F, t9272: F, t9273: F, t18313: F, t31119: F, t3394: F, t35180: F, t9562: F) -> (F, F, F, F, F, F) {
    let t41643 = t1415 * t10417 * t7030;
    let t41645 = t31051 * t12960;
    let t41649 = t6576 * t10473 * t2478;
    let t41656 = t9272 * t34688 * t9273;
    let t41660 = t31119 * t18313 * t3394 * t9273;
    let t41666 = t35180 * t9562;
    (t41643, t41645, t41649, t41656, t41660, t41666)
}
