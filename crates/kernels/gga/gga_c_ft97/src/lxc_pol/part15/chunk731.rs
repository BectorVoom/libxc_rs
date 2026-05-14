//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 731/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk731<F: Float>(t245: F, t1178: F, t20044: F, t21: F, t21780: F, t267: F, t4431: F, t5: F, t5186: F, t920: F, t1273: F, t5478: F, t332: F, t113: F, t5473: F, t4381: F, t1274: F, t4635: F) -> (F, F, F, F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t21794 = piecewise3(t246, 0.0, t5 * t21780 * t21 / 4.0 + 3.0 / 4.0 * t5 * t5186 * t920 + 3.0 / 4.0 * t5 * t1178 * t4431 + t5 * t267 * t20044 / 4.0);
    let t21800 = t5478 * t1273;
    let t21801 = t21800 * t332;
    let t21802 = t21801 * t113;
    let t21805 = t5473 * t1273;
    let t21806 = t21805 * t4381;
    let t21812 = t1274 * t4635;
    (t21794, t21801, t21802, t21805, t21806, t21812)
}
