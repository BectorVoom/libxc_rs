//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 635/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk635<F: Float>(t1429: F, t2435: F, t1428: F, t2777: F, t2439: F, t1385: F, t225: F) -> (F, F, F, F) {
    let t4082 = 0.73171657588172351096e-2 * t2435 * t1429;
    let t4083 = t2777 * t1428;
    let t4085 = 0.65049603595885220126e-3 * t2439 * t4083;
    let t4086 = t225 * t1385;
    (t4082, t4083, t4085, t4086)
}
