//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 781/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk781<F: Float>(t10019: F, t4101: F, t555: F, t5744: F, t786: F, t3923: F, t675: F, t268: F, t4003: F, t2435: F, t4093: F, t4083: F, t9303: F) -> (F, F, F, F, F) {
    let t10020 = t4101 * t10019;
    let t10022 = t5744 * t555;
    let t10023 = t786 * t10022;
    let t10024 = t675 * t3923;
    let t10026 = t268 * t10024 * t4003;
    let t10027 = t10023 * t10026;
    let t10032 = t2435 * t4093;
    let t10035 = F::new(0.26019841438354088051e-2) * t9303 * t4083;
    (t10020, t10024, t10027, t10032, t10035)
}
