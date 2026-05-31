//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2159/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2159<F: Float>(t2322: F, t30005: F, t4254: F, t1310: F, t30004: F, t651: F, t27123: F, t7742: F, t27126: F, t28063: F, t7732: F, t28056: F, t4248: F) -> (F, F, F, F, F, F, F) {
    let t108078 = F::cast_from(2.0_f64) * t2322 * t30005;
    let t108080 = F::cast_from(2.0_f64) * t4254 * t30005;
    let t108083 = F::cast_from(2.0_f64) * t651 * t1310 * t30004;
    let t108085 = F::cast_from(4.0_f64) * t27123 * t7742;
    let t108087 = F::cast_from(4.0_f64) * t27126 * t7742;
    let t108089 = F::cast_from(4.0_f64) * t7732 * t28063;
    let t108099 = F::cast_from(4.0_f64) * t4248 * t28056;
    (t108078, t108080, t108083, t108085, t108087, t108089, t108099)
}
