//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1132/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1132(t111: f64, t5363: f64, t1851: f64, t671: f64, t1372: f64, t794: f64, t213: f64, t225: f64, t1887: f64, t22797: f64, t268: f64, t547: f64, t6559: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t55353 = t5363 * t111;
    let t75795 = t1851 * t671;
    let t80645 = t794 * t1372;
    let t80650 = t213 * t1372 * t225;
    let t81159 = t22797 * t1887;
    let t81228 = t6559 * t547 * t268;
    (t55353, t75795, t80645, t80650, t81159, t81228)
}
