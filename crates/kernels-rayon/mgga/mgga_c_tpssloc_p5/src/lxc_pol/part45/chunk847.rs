//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 847/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk847(t12020: f64, t2091: f64, t1911: f64, t857: f64, t776: f64, t23270: f64, t22986: f64, t2717: f64, t865: f64, t1888: f64, t794: f64, t8331: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26989 = t12020 * t2091;
    let t30622 = t857 * t1911;
    let t30623 = t30622 * t776;
    let t30624 = t23270 * t30623;
    let t30626 = 0.3289868133696452873e-1_f64 * t22986 * t30624;
    let t30633 = t2717 * t1911;
    let t30634 = t30633 * t865;
    let t30635 = t23270 * t30634;
    let t30637 = 0.3289868133696452873e-1_f64 * t1888 * t30635;
    let t30638 = t794 * t8331;
    (t26989, t30622, t30623, t30624, t30626, t30633, t30634, t30635, t30637, t30638)
}
