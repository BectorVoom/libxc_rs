//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1697/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1697<F: Float>(t2097: F, t785: F, t1358: F, t2439: F, t2435: F, t7493: F, t26069: F, t26277: F, t26072: F, t7515: F, t116: F, t7356: F) -> (F, F, F, F, F, F, F) {
    let t26358 = t785 * t2097;
    let t26359 = t26358 * t1358;
    let t26361 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t26359;
    let t26363 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t7493;
    let t26365 = F::cast_from(0.22849835011101738147e-2_f64) * t26069 * t26277;
    let t26366 = t26072 * t7515;
    let t26399 = t7356 * t116;
    (t26358, t26359, t26361, t26363, t26365, t26366, t26399)
}
