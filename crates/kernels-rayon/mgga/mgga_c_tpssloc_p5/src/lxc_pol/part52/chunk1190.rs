//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1190/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1190(t1874: f64, t24932: f64, t27888: f64, t6525: f64, t7266: f64, t2314: f64, t8675: f64, t4034: f64, t1873: f64, t7408: f64, t652: f64, t671: f64, t8682: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31898 = t24932 * t1874;
    let t31900 = t27888 * t1874;
    let t31902 = t7266 * t6525;
    let t31904 = t2314 * t8675;
    let t31906 = t4034 * t8675;
    let t31908 = t7408 * t1873;
    let t31909 = t652 * t31908;
    let t31913 = t8682 * t671;
    (t31898, t31900, t31902, t31904, t31906, t31908, t31909, t31913)
}
