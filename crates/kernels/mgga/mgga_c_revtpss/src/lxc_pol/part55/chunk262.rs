//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 262/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk262<F: Float>(t1209: F, t487: F, t225: F, t494: F, t1118: F, t1124: F) -> (F, F, F, F) {
    let t1210 = t1209 * t487;
    let t1211 = t225 * t494;
    let t1212 = F::cast_from(0.14816666666666666667e-1_f64) * t1118;
    let t1214 = -t1212 + F::cast_from(0.14816666666666666667e-1_f64) * t1124;
    (t1210, t1211, t1212, t1214)
}
