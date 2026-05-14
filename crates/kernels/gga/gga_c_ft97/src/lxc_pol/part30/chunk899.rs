//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 899/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk899<F: Float>(t11: F, t1109: F, t171: F, t33446: F, t173: F, t35453: F, t35454: F, t35455: F, t150522: F, t3766: F, t141058: F, t33424: F, t35361: F, t2426: F, t3771: F, t6793: F) -> (F, F, F, F, F, F) {
    let t150546 = t11 * t1109 * t171;
    let t150547 = t150546 * t33446;
    let t150552 = t35453 * t35454 * t173 * t35455;
    let t150554 = t3766 * t150522;
    let t150558 = t33424 * t141058 * t35361;
    let t150565 = t3771 * t2426 * t6793 * t171;
    (t150546, t150547, t150552, t150554, t150558, t150565)
}
