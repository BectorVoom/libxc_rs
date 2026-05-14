//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1141/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1141<F: Float>(t22914: F, t25606: F, t25612: F, t25590: F, t8466: F, t25595: F, t38664: F, t6547: F, t25863: F, t23089: F, t6414: F, t100050: F, t100055: F, t11437: F, t11827: F, t11982: F, t1564: F, t1643: F, t1904: F, t22907: F, t22908: F, t25609: F, t25610: F, t25615: F, t5501: F, t6421: F, t91504: F, t925: F, t93871: F, t93910: F) -> (F, F, F, F) {
    let t100065 = 2.0 / 27.0 * t22914 * t25606;
    let t100067 = 2.0 / 27.0 * t22914 * t25612;
    let t100072 = t8466 * t25590;
    let t100074 = t8466 * t25595;
    let t100076 = t38664 * t6547;
    let t100079 = t22914 * t25863 / 27.0;
    let t100085 = t6414 * t23089 / 9.0;
    let t100086 = t5501 * t25609 * t25610 * t11982 / 9.0 + 2.0 / 9.0 * t5501 * t25615 * t100050 * t11437 + 2.0 / 9.0 * t5501 * t22907 * t100055 * t1904 + 2.0 / 27.0 * t5501 * t93871 * t6421 * t1643 - t100065 - t100067 - t5501 * t91504 * t22908 * t11827 / 3.0 + 8.0 * t100072 + 8.0 * t100074 + 4.0 * t100076 + t100079 - t5501 * t1564 * t93910 * t925 / 18.0 - t100085;
    (t100072, t100074, t100076, t100086)
}
