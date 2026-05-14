//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1113/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1113<F: Float>(t1637: F, t6687: F, t89: F, t26864: F, t8392: F, t5773: F, t9132: F, t40465: F, t22914: F, t29616: F, t100065: F, t100067: F, t100079: F, t1286: F, t1557: F, t1570: F, t16169: F, t22907: F, t25558: F, t25577: F, t25606: F, t25609: F, t25610: F, t25612: F, t25615: F, t25618: F, t25847: F, t28: F, t29465: F, t29734: F, t3188: F, t379: F, t5495: F, t5501: F, t984: F) -> (F, F, F, F, F) {
    let t107685 = t89 * t1637 * t6687;
    let t107691 = 4.0 / 81.0 * t8392 * t26864;
    let t107703 = t9132 * t5773;
    let t107707 = t40465 * t5773;
    let t115114 = t22914 * t29616;
    let t115130 = t5501 * t22907 * t29734 * t379 / 9.0 - 2.0 / 27.0 * t25558 * t25618 + 2.0 / 9.0 * t5501 * t25609 * t984 * t1570 * t3188 - 2.0 / 27.0 * t5501 * t25615 * t984 * t1557 * t3188 - 2.0 / 27.0 * t115114 - t100065 - t100067 + 2.0 / 9.0 * t25558 * t25606 + 2.0 / 9.0 * t25558 * t25612 + 4.0 / 9.0 * t25577 * t25609 * t25610 * t16169 + t5495 * t29465 / 3.0 + t1286 * t28 * t25847 * t984 / 3.0 + t100079;
    (t107685, t107691, t107703, t107707, t115130)
}
