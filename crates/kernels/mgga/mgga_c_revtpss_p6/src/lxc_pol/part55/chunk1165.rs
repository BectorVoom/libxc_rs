//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1165/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1165<F: Float>(t124235: F, t32802: F, t606: F, t8442: F, t32806: F, t33281: F, t239: F, t8441: F, t8621: F, t8737: F, t32795: F, t2172: F, t7541: F) -> (F, F, F, F, F) {
    let t124238 = t32802 * t8442 * t124235 * t606;
    let t124246 = t32806 * t33281;
    let t124255 = F::cast_from(55.0_f64) / F::cast_from(81.0_f64) * t8737 * t8621 * t8441 * t239;
    let t124256 = t32795 * t33281;
    let t124411 = t7541 * t2172;
    (t124238, t124246, t124255, t124256, t124411)
}
