//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 963/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk963<F: Float>(t1544: F, t18268: F, t18850: F, t198: F, t23106: F, t23110: F, t23111: F, t23114: F, t23123: F, t23124: F, t23127: F, t23128: F, t23129: F, t23130: F, t23148: F, t2403: F, t262: F, t4541: F, t765: F, t9394: F) -> F {
    let t23152 = -F::cast_from(9.0_f64) * t1544 * t18268 * t2403 + F::cast_from(9.0_f64) * t1544 * t18850 * t2403 + F::cast_from(6.0_f64) * t198 * t23114 * t262 + F::cast_from(3.0_f64) * t198 * t23148 * t765 + F::cast_from(18.0_f64) * t23111 * t4541 + F::cast_from(18.0_f64) * t23124 * t4541 - t23106 + t23110 + t23123 + t23127 + t23128 + t23129 + t23130 + t9394;
    t23152
}
