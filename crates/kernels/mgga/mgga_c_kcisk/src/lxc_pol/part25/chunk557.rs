//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 557/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk557<F: Float>(t1791: F, t5043: F, t1801: F, t4648: F, t1800: F, t1799: F, t1693: F, t1792: F, t4583: F, t4794: F, t4800: F, t4806: F, t4809: F, t4812: F, t4814: F, t4819: F, t4823: F, t4827: F, t4830: F, t671: F) -> (F, F, F, F, F) {
    let t5044 = t5043 * t1791;
    let t5048 = t1801 * t4648;
    let t5049 = t1800 * t5048;
    let t5050 = t1799 * t5049;
    let t5052 = 0.33163888888888888888e-2 * t4583 - 0.24872916666666666666e-2 * t4800 + 0.16581944444444444444e-2 * t4806 - t4809 - 0.33163888888888888888e-2 * t4812 + 0.22109259259259259258e-2 * t4814 - 0.49745833333333333332e-2 * t4819 + 0.74498e-1 * t4823 * t4827 - 0.386e0 * t4830 * t1792 - 0.193e0 * t1693 * t5044 + t4794 * t671 + 0.16581944444444444444e-2 * t5050;
    (t5044, t5048, t5049, t5050, t5052)
}
