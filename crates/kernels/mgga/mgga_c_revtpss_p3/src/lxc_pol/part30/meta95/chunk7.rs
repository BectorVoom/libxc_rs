//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 612/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk612<F: Float>(t265: F, t502: F, t2144: F, t2149: F, t2152: F, t460: F, t1300: F, t198: F, t1995: F, t336: F) -> (F, F) {
    let t503 = t265 < t502;
    let t2155 = F::cast_from(0.65854491829355115987e0_f64) * t460 * t2144 - F::cast_from(0.4336814094102599731e0_f64) * t2149 * t2152;
    let t2159 = piecewise3::<F>(t503, t198 * t336 * t2155 * t1300, t1995);
    (t2155, t2159)
}
