//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1282/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1282<F: Float>(t10002: F, t30933: F, t1882: F, t31298: F, t31293: F, t13830: F, t6940: F, t111045: F, t111047: F, t123814: F, t124658: F, t14127: F, t1456: F, t18123: F, t18646: F, t18749: F, t1901: F, t242: F, t2574: F, t265: F, t28128: F, t28276: F, t30859: F, t3977: F, t4005: F, t446: F, t6154: F, t6852: F, t724: F, t729: F, t773: F, t97952: F) -> (F, F, F) {
    let t124662 = t10002 * t30933;
    let t124674 = t1882 * t31298;
    let t124684 = t1882 * t31293;
    let t124686 = t13830 * t6940;
    let t124690 = 2.0 / 3.0 * t446 * t729 * t3977 * t28276 - t446 * t724 * t1456 * t18123 / 9.0 + 4.0 / 3.0 * t446 * t2574 * t4005 * t6852 + 4.0 / 81.0 * t97952 - t446 * t242 * t124658 / 3.0 + 4.0 / 3.0 * t446 * t242 * t124662 + t446 * t729 * t6154 * t18646 / 3.0 - 4.0 / 3.0 * t1901 * t14127 * t28128 * t18749 - 2.0 / 9.0 * t124674 - t446 * t729 * t773 * t30859 / 3.0 - t446 * t729 * t265 * t123814 / 3.0 + t124684 / 9.0 - 2.0 / 3.0 * t446 * t242 * t124686 + t111045 + t111047;
    (t124662, t124686, t124690)
}
