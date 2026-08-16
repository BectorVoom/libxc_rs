//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1143/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1143(t1911: f64, t857: f64, t776: f64, t23270: f64, t22986: f64, t6662: f64, t2718: f64, t2717: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30622 = t857 * t1911;
    let t30623 = t30622 * t776;
    let t30624 = t23270 * t30623;
    let t30626 = 0.3289868133696452873e-1_f64 * t22986 * t30624;
    let t30629 = t1911 * t6662;
    let t30630 = t2718 * t30629;
    let t30633 = t2717 * t1911;
    (t30622, t30623, t30624, t30626, t30630, t30633)
}
