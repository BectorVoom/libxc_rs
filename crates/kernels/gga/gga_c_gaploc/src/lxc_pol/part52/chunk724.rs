//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 724/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk724<F: Float>(t14361: F, t14395: F, t14402: F, t14406: F, t13338: F, t13345: F, t13352: F, t13573: F, t13587: F, t14292: F, t14297: F, t14349: F, t14350: F, t1960: F, t2969: F, t3749: F, t748: F) -> (F, F) {
    let t14408 = t14361 + t14395 + t14402 + t14406;
    let t14412 = F::new(4.0) * t14350 * t1960 - t14408 * t748 - F::new(2.0) * t2969 * t3749 - t13338 + t13345 - t13352 + t13573 + t13587 + t14292 - t14297 + t14349;
    (t14408, t14412)
}
