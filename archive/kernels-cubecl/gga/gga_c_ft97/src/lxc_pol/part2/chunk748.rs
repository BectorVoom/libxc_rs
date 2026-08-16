//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 748/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk748<F: Float>(t1557: F, t469: F, t3188: F, t432: F, t26: F, t356: F, t1570: F, t1800: F, t942: F, t1565: F, t11718: F, t11720: F, t11724: F, t11728: F, t11732: F, t11734: F, t11735: F, t11738: F, t11741: F, t11745: F, t11746: F, t11749: F, t11755: F, t3139: F, t462: F, t8289: F, t8298: F, t8301: F, t8302: F, t8331: F) -> (F, F) {
    let t11756 = t469 * t1557;
    let t11757 = t3188 * t432;
    let t11758 = t11756 * t11757;
    let t11761 = t26 * t356;
    let t11762 = t469 * t1570;
    let t11763 = t11762 * t11757;
    let t11766 = t1800 * t942;
    let t11767 = t11766 * t1565;
    let t11771 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t11718 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11720 + F::cast_from(4.0_f64) * t462 * t11724 - F::cast_from(6.0_f64) * t462 * t11728 - t11732 - t8301 - t11734 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t11735 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t3139 * t11738 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3139 * t11741 - t11745 - F::cast_from(2.0_f64) * t462 * t11746 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t11749 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t8289 + t8298 / F::cast_from(3.0_f64) - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t8302 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t11755 * t11758 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11761 * t11763 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t11761 * t11767 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8331;
    (t11761, t11771)
}
