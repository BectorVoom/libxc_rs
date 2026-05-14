//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1263/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1263<F: Float>(t21222: F, t3626: F, t1222: F, t1227: F, t13012: F, t17593: F, t17619: F, t17622: F, t21200: F, t21203: F, t21210: F, t21213: F, t21216: F, t21219: F, t3625: F, t5340: F, t5369: F, t5373: F, t5384: F, t5386: F) -> (F,) {
    let t21223 = t3626 * t21222;
    let t21226 = t17593 + 0.85748036236139473944e-3 * t5384 * t21200 - 0.45732285992607719436e-2 * t21203 * t5386 + t13012 / 1296.0 - t17619 - t17622 + t5373 * t5369 / 54.0 - t1222 * t21210 / 288.0 - 11.0 / 324.0 * t21213 * t1227 - 0.19055119163586549765e-3 * t21216 - 0.14291339372689912324e-3 * t3625 * t21219 - 0.28582678745379824648e-3 * t5340 * t21223;
    (t21226,)
}
