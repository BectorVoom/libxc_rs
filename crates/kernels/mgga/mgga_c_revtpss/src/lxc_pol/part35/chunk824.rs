//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 824/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk824<F: Float>(t14336: F, t14339: F, t1544: F, t18860: F, t5966: F) -> (F, F, F, F) {
    let t23106 = 0.51947577317044391276e2 * t14336;
    let t23110 = 0.73245789224026180216e-3 * t14339;
    let t23111 = t18860 * t1544;
    let t23114 = t5966 * t1544;
    (t23106, t23110, t23111, t23114)
}
