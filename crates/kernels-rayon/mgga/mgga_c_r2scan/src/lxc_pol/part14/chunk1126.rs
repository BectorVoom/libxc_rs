//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1126/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1126(t132: f64, t1567: f64, t39613: f64, t7340: f64, t1054: f64, t6132: f64, t7345: f64, t6139: f64, t10872: f64, t11686: f64, t10891: f64, t11748: f64) -> (f64, f64, f64, f64, f64) {
    let t39614 = t132 * t1567;
    let t39616 = t39613 * t39614 * t7340;
    let t39619 = t6132 * t1054 * t7345;
    let t39622 = t6139 * t1054 * t7340;
    let t39627 = t10872 * t11686;
    let t39629 = t11748 * t10891;
    (t39616, t39619, t39622, t39627, t39629)
}
