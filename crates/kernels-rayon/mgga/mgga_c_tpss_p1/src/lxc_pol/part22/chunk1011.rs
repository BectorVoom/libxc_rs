//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1011/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1011(t10513: f64, t10562: f64, t10918: f64, t10935: f64, t2: f64, t826: f64, t555: f64, t259: f64, t22: f64, t3742: f64, t2614: f64, t3899: f64) -> (f64, f64, f64, f64, f64) {
    let t10937 = t10513 + t10562 + t10918 + t10935;
    let t10945 = t826 * t2;
    let t10947 = 2.0_f64 * t10945 * t555;
    let t10948 = t259 * t555;
    let t10950 = 3.0_f64 * t3742 * t22;
    let t10952 = t3899 * t2614;
    (t10937, t10947, t10948, t10950, t10952)
}
