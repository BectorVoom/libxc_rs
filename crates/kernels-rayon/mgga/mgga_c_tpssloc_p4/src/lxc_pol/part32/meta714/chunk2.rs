//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2245/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2245(t22690: f64, t23122: f64, t5544: f64, t841: f64, t23097: f64, t5617: f64, t776: f64, t815: f64, t1510: f64, t4233: f64, t6605: f64, t232: f64, t58688: f64) -> (f64, f64, f64, f64) {
    let t98647 = t23122 * t22690 * t841 * t5544;
    let t98651 = t23097 * t815 * t5617 * t776;
    let t98655 = t6605 * t815 * t1510 * t4233;
    let t98659 = t6605 * t815 * t58688 * t232;
    (t98647, t98651, t98655, t98659)
}
