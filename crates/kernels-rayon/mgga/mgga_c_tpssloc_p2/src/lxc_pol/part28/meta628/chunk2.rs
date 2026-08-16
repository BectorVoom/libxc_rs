//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1969/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1969(t193: f64, t201: f64, t7844: f64, t1877: f64, t2057: f64, t2249: f64, t22951: f64, t22961: f64, t22968: f64, t23299: f64, t24191: f64, t25024: f64, t2522: f64, t25366: f64, t26563: f64, t26744: f64, t4314: f64, t7110: f64, t7114: f64, t7845: f64, t84797: f64, t86710: f64, t86746: f64, t86782: f64, t86803: f64, t86816: f64, t86825: f64, t87981: f64, t87994: f64) -> (f64, f64) {
    let t92319 = t193 * t201 * t7844;
    let t92349 = -3.0_f64 * t24191 * t86710 + 3.0_f64 * t2522 * t7110 * t25024 - 3.0_f64 * t84797 * t25366 - 3.0_f64 * t92319 * t22961 - t1877 * t7114 * t87994 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t86816 + 3.0_f64 * t4314 * t7845 * t22951 + 3.0_f64 * t4314 * t2057 * t86825 + t1877 * t7845 * t2249 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t86803 + 3.0_f64 / 2.0_f64 * t2522 * t7845 * t22968 + 6.0_f64 * t24191 * t86782 - t1877 * t7114 * t86746 - t1877 * t26744 * t23299 + 3.0_f64 * t26563 * t87981;
    (t92319, t92349)
}
