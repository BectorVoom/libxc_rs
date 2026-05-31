//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1088/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1088<F: Float>(t1882: F, t34644: F, t34703: F, t34511: F, t487: F, t103510: F, t110: F, t11490: F, t11593: F, t11902: F, t1339: F, t137797: F, t138057: F, t144991: F, t1901: F, t1902: F, t25595: F, t25846: F, t26166: F, t26372: F, t26436: F, t3052: F, t3214: F, t3219: F, t32597: F, t32598: F, t34379: F, t446: F, t452: F, t47659: F, t499: F, t5617: F, t6564: F, t7281: F, t8411: F, t8417: F) -> (F, F, F, F) {
    let t146680 = t1882 * t34644;
    let t146682 = t1882 * t34703;
    let t146693 = t487 * t34511;
    let t146741 = t1901 * t11902 * t32598 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t11593 * t1902 * t32597 * t3052 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t11490 * t137797 * t3214 - F::cast_from(2.0_f64) * t1901 * t26372 * t8417 * t7281 * t3219 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11490 * t26166 * t25595 - F::cast_from(2.0_f64) * t446 * t8411 * t499 * t34379 - F::cast_from(2.0_f64) * t446 * t8411 * t110 * t144991 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t6564 * t5617 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t1339 * t25846 - t138057 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t47659 * t103510 * t26436;
    (t146680, t146682, t146693, t146741)
}
