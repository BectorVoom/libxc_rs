//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1034/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1034(t23237: f64, t8335: f64, t1880: f64, t6547: f64, t8357: f64, t1902: f64, t234: f64, t776: f64, t6637: f64, t6552: f64, t794: f64, t8356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30671 = t23237 * t8335;
    let t30673 = 0.16449340668482264365e-1_f64 * t1880 * t30671;
    let t30675 = 0.38381794893125283518e-1_f64 * t6547 * t8357;
    let t30676 = t234 * t1902;
    let t30677 = t30676 * t776;
    let t30678 = t6637 * t30677;
    let t30680 = 0.3289868133696452873e-1_f64 * t6552 * t30678;
    let t30681 = t794 * t8356;
    (t30671, t30673, t30675, t30676, t30677, t30678, t30680, t30681)
}
