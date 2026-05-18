//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 753/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk753<F: Float>(t1695: F, t2517: F, t1699: F, t926: F, t7088: F, t7090: F, t7093: F, t7096: F, t2524: F, t471: F, t64: F, t90: F, t931: F) -> (F, F, F, F) {
    let t7098 = t2517 * t1695;
    let t7100 = t926 * t1699;
    let t7102 = F::new(189.0) / F::new(512.0) * t7088 - F::new(483.0) / F::new(16384.0) * t7090 + F::new(147.0) / F::new(1048576.0) * t7093 - F::new(49.0) / F::new(1048576.0) * t7096 + F::new(161.0) / F::new(16384.0) * t7098 - F::new(63.0) / F::new(512.0) * t7100;
    let t7112 = t7102 * t471 - F::new(8.0) / F::new(3.0) * t2524 * t64 + F::new(4.0) / F::new(3.0) * t931 * t90 + F::new(63.0) / F::new(512.0) * t7088 - F::new(49.0) / F::new(16384.0) * t7090 + F::new(49.0) / F::new(49152.0) * t7098 - F::new(21.0) / F::new(512.0) * t7100;
    (t7098, t7100, t7102, t7112)
}
