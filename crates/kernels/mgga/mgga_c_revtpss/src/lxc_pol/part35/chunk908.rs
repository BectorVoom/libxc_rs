//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 908/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk908<F: Float>(t18556: F, t10566: F, t23104: F, t23106: F, t23110: F, t23123: F, t23127: F, t23128: F, t23129: F, t23130: F, t9394: F, t18563: F) -> (F, F, F) {
    let t23186 = F::cast_from(0.54934341918019635162e-3_f64) * t18556;
    let t23187 = -t23104 - t23106 + t23110 + t23123 + t23127 + t23128 + t23129 + t9394 + t23130 + t10566 - t23186;
    let t23189 = F::cast_from(0.17544670867903938621e1_f64) * t18563;
    (t23186, t23187, t23189)
}
