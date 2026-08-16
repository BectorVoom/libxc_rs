//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1210/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1210<F: Float>(t19231: F, t19261: F, t1256: F, t18247: F, t18249: F, t18251: F, t18257: F, t18261: F, t18264: F, t18268: F, t18270: F, t18273: F, t18278: F, t18282: F, t18285: F, t18672: F, t18676: F, t18679: F, t18909: F, t18913: F, t193: F, t336: F, t4700: F, t5091: F, t5095: F) -> F {
    let t19262 = t19231 + t19261;
    let t19266 = t1256 * t19262 * t193 * t336 - F::cast_from(2.0_f64) * t4700 * t5091 * t5095 - t18247 - t18249 - t18251 - t18257 + t18261 + t18264 + t18268 - t18270 - t18273 - t18278 + t18282 - t18285 - t18672 + t18676 + t18679 + t18909 - t18913;
    t19266
}
