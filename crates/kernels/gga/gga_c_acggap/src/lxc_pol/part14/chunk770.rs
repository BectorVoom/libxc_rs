//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 770/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk770<F: Float>(t336: F, t5630: F, t570: F, t7806: F, t7850: F, t7854: F, t7863: F, t8953: F, t8975: F, t8983: F, t9348: F, t9356: F, t9359: F, t9739: F, t9741: F, t9743: F, t9747: F, t9749: F, t9751: F, t9753: F, t9755: F, t9759: F) -> (F, F) {
    let t9761 = t336 * t5630;
    let t9762 = t570 * t9761;
    let t9764 = -t7806 - t9348 - 0.31448092289604152068e-3 * t8953 - t9739 / 24.0 - t9741 / 48.0 + t9743 / 16.0 + t9356 - 0.56606566121287473722e-2 * t8975 - t9359 + 0.25724410870841842184e-2 * t8983 - t9747 / 48.0 - t9749 / 96.0 + t9751 / 48.0 + 0.85748036236139473945e-2 * t9753 + 0.25724410870841842183e-2 * t9755 + t7850 + t7854 - 0.10718504529517434243e-3 * t9759 + t7863 + t9762 / 96.0;
    (t9761, t9764)
}
