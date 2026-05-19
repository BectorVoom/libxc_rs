//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 699/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk699<F: Float>(t7286: F, t7289: F, t1419: F, t1955: F, t7282: F) -> (F, F, F) {
    let t7291 = F::cast_from(0.12851425765524037203e-1_f64) * t7289 * t7286;
    let t7292 = t1955 * t1419;
    let t7295 = t1955 * t7282;
    (t7291, t7292, t7295)
}
