//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 978/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk978<F: Float>(t10051: F, t1160: F, t3951: F, t737: F, t265: F, t42109: F, t2486: F, t2568: F, t676: F, t754: F, t2567: F, t10002: F, t9802: F, t2492: F, t773: F, t9895: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t51340 = t1160 * t10051;
    let t51609 = t737 * t3951;
    let t51669 = t42109 * t265;
    let t51687 = t2486 * t2568;
    let t51853 = t676 * t754;
    let t51892 = t3951 * t2567;
    let t51901 = t737 * t10002;
    let t51990 = t9802 * t1160;
    let t52002 = t2492 * t3951;
    let t52018 = t9895 * t773;
    (t51340, t51609, t51669, t51687, t51853, t51892, t51901, t51990, t52002, t52018)
}
