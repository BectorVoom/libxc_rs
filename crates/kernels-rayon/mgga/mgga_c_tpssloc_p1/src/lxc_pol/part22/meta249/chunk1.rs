//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1362/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1362(t10868: f64, t248: f64, t884: f64, t1041: f64, t10478: f64, t3128: f64, t10472: f64) -> (f64, f64, f64, f64) {
    let t10870 = t248 * t10868 * t884;
    let t10871 = t1041 * t10870;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    (t10870, t10871, t10875, t10876)
}
