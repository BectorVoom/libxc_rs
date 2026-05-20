//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 927/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk927<F: Float>(t2482: F, t27: F, t4000: F, t555: F, t5744: F, t786: F, t4083: F, t9303: F, t123: F, t212: F, t2434: F) -> (F, F, F, F, F) {
    let t10001 = t2482 * t4000 * t27;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10035 = F::cast_from(0.26019841438354088051e-2_f64) * t9303 * t4083;
    let t10069 = t123 * t2434 * t212;
    (t10001, t10022, t10023, t10035, t10069)
}
