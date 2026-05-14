//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 843/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk843<F: Float>(t280: F, t39: F, t2035: F, t1109: F, t1208: F, t820: F, t14722: F, t1196: F, t817: F, t800: F, t4100: F, t7853: F, t4092: F, t811: F, t14752: F, t4064: F, t4125: F) -> (F, F, F, F, F, F, F, F) {
    let t19038 = t280 * t39;
    let t19039 = t19038 * t2035;
    let t19043 = t1109 * t1208;
    let t19044 = t19043 * t820;
    let t19045 = t14722 * t19044;
    let t19048 = t817 * t1196;
    let t19049 = t800 * t19048;
    let t19050 = t7853 * t4100;
    let t19053 = t4092 * t19048;
    let t19056 = t19043 * t811;
    let t19057 = t14722 * t19056;
    let t19066 = t14752 * t1208;
    let t19069 = t4064 * t4125;
    (t19039, t19045, t19049, t19050, t19053, t19057, t19066, t19069)
}
