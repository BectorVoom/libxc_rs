//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 601/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk601<F: Float>(t265: F, t393: F, t1978: F, t1983: F, t1986: F, t342: F, t1962: F, t207: F, t198: F, t892: F, t1102: F, t336: F) -> (F, F, F, F) {
    let t394 = t265 < t393;
    let t1989 = F::cast_from(0.65854491829355115987e0_f64) * t342 * t1978 - F::cast_from(0.4336814094102599731e0_f64) * t1983 * t1986;
    let t1993 = t207 * t1962;
    let t1995 = t198 * t1993 * t892;
    let t1996 = piecewise3::<F>(t394, t198 * t336 * t1989 * t1102, t1995);
    (t1989, t1993, t1995, t1996)
}
