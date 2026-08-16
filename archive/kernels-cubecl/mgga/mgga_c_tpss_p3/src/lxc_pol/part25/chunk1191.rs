//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1191/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1191<F: Float>(t19407: F, t77: F, t1290: F, t1976: F, t3426: F, t578: F, t3432: F, t1630: F, t18436: F, t136: F, t527: F, t1693: F) -> (F, F, F, F, F, F, F) {
    let t19408 = t77 * t19407;
    let t19411 = t1976 * t1290;
    let t19414 = t578 * t3426;
    let t19417 = t578 * t3432;
    let t19466 = t18436 * t1630;
    let t19468 = t527 * t136;
    let t19469 = t1693 * t19468;
    (t19408, t19411, t19414, t19417, t19466, t19468, t19469)
}
