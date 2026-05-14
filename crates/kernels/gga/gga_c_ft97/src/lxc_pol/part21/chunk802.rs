//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 802/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk802<F: Float>(t2001: F, t5820: F, t23824: F, t5818: F, t1354: F, t1995: F, t23809: F, t527: F) -> (F, F, F, F, F) {
    let t23839 = t2001 * t5820;
    let t23842 = t5818 * t23824;
    let t23847 = t1995 * t1354;
    let t23866 = t1995 * t23809;
    let t23869 = t527 * t1354;
    (t23839, t23842, t23847, t23866, t23869)
}
