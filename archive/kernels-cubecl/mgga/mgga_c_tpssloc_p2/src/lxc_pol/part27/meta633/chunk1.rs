//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2132/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2132<F: Float>(t81575: F, t25038: F, t4282: F, t6646: F, t9647: F, t25251: F, t87049: F, t23012: F, t7529: F, t13380: F, t22986: F, t2647: F) -> (F, F, F, F, F) {
    let t87073 = F::cast_from(0.3289868133696452873e-1_f64) * t81575;
    let t87076 = t25038 * t6646 * t4282 * t9647;
    let t87078 = t87049 * t25251;
    let t87080 = t23012 * t7529;
    let t87084 = t22986 * t6646 * t13380 * t2647;
    (t87073, t87076, t87078, t87080, t87084)
}
