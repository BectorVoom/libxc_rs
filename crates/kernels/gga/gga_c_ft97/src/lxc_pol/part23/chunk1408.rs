//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1408/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1408<F: Float>(t5309: F, t98724: F, t113780: F, t114818: F, t114827: F, t114837: F, t114839: F, t11593: F, t125560: F, t126799: F, t126833: F, t126837: F, t126859: F, t15290: F, t18123: F, t18997: F, t1901: F, t19465: F, t25037: F, t28516: F, t2874: F, t296: F, t4176: F, t44518: F, t446: F, t53797: F, t56339: F, t56643: F, t6273: F, t7105: F, t72190: F, t72443: F) -> (F, F) {
    let t128448 = t98724 * t5309;
    let t128455 = 8.0 / 3.0 * t1901 * t72190 * t7105 * t4176 - 4.0 / 9.0 * t1901 * t72443 * t28516 + 4.0 / 27.0 * t114818 - 4.0 / 27.0 * t1901 * t56643 * t126799 - 2.0 / 27.0 * t1901 * t44518 * t25037 * t19465 - 4.0 / 9.0 * t1901 * t15290 * t126859 + 10.0 / 81.0 * t1901 * t56339 * t126833 - 8.0 / 27.0 * t11593 * t15290 * t126837 + 4.0 / 9.0 * t53797 * t113780 * t18997 - 4.0 / 27.0 * t114827 + t1901 * t2874 * t6273 * t18123 / 9.0 + 2.0 / 3.0 * t446 * t296 * t128448 - 2.0 / 3.0 * t446 * t296 * t125560 + t114837 + t114839;
    (t128448, t128455)
}
