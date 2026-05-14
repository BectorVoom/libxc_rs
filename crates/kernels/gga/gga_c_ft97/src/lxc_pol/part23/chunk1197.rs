//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1197/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1197<F: Float>(t13927: F, t27983: F, t31060: F, t761: F, t31107: F, t8392: F, t18712: F, t24519: F, t18459: F, t110950: F, t111048: F, t11593: F, t13839: F, t14200: F, t1424: F, t18386: F, t18412: F, t18438: F, t18442: F, t18506: F, t1901: F, t242: F, t2606: F, t28246: F, t28360: F, t31098: F, t3887: F, t3977: F, t42362: F, t446: F, t51669: F, t53797: F, t54032: F, t684: F, t713: F, t729: F, t762: F, t97470: F, t97705: F) -> (F, F, F, F) {
    let t122179 = t13927 * t27983;
    let t122193 = t761 * t31060;
    let t122205 = t8392 * t31107;
    let t122215 = t24519 * t18712;
    let t122219 = t24519 * t18459;
    let t122227 = 2.0 / 3.0 * t446 * t729 * t3977 * t28246 + 4.0 / 3.0 * t446 * t242 * t122179 + 4.0 / 9.0 * t53797 * t97705 * t18412 - 4.0 / 27.0 * t97470 + 8.0 / 9.0 * t53797 * t111048 * t18438 - 8.0 / 27.0 * t54032 * t111048 * t18442 + t1901 * t2606 * t122193 * t684 / 9.0 + 4.0 / 9.0 * t1901 * t110950 * t3887 - t446 * t729 * t31098 * t713 / 3.0 - t122205 / 27.0 + t446 * t729 * t762 * t1424 * t18386 / 3.0 - 4.0 / 9.0 * t11593 * t13839 * t28360 + 2.0 / 27.0 * t1901 * t14200 * t122215 - 4.0 / 27.0 * t1901 * t51669 * t122219 - 2.0 / 27.0 * t1901 * t42362 * t24519 * t18506;
    (t122179, t122215, t122219, t122227)
}
