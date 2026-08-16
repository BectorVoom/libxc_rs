//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 987/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk987<F: Float>(t10103: F, t1432: F, t686: F, t136: F, t1419: F, t2457: F, t3964: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F) -> (F, F, F, F, F) {
    let t10105 = t1432 * t10103 * t686;
    let t10107 = t1419 * t136;
    let t10109 = t3964 * t10107 * t2457;
    let t10111 = t9646 * t225;
    let t10114 = F::cast_from(0.19637199382202157274e-3_f64) * t10111 * t1428 * t22;
    let t10115 = t22 * t2452;
    (t10105, t10109, t10111, t10114, t10115)
}
