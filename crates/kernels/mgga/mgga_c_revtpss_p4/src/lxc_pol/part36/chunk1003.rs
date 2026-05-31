//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1003/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1003<F: Float>(t23400: F, t23420: F, t10566: F, t10568: F, t10577: F, t10582: F, t10584: F, t10586: F, t1583: F, t18865: F, t1940: F, t198: F, t207: F, t23186: F, t23189: F, t892: F, t9514: F, t9517: F, t9521: F) -> (F, F) {
    let t23421 = t23400 + t23420;
    let t23428 = t198 * t207 * t23421 * t892 - F::cast_from(3.0_f64) * t1583 * t18865 * t1940 + t10566 - t10568 + t10577 + t10582 - t10584 - t10586 - t23186 - t23189 + t9514 - t9517 - t9521;
    (t23421, t23428)
}
