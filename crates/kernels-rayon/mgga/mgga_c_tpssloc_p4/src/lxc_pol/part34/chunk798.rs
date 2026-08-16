//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 798/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk798(t17763: f64, t973: f64, t2970: f64, t5828: f64, t10231: f64, t5817: f64, t2989: f64, t5398: f64, t2987: f64, t5836: f64, t5842: f64, t13847: f64, t4514: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17764 = t973 * t17763;
    let t17769 = t2970 * t5828;
    let t17770 = t973 * t17769;
    let t17783 = t10231 * t5817;
    let t17784 = t973 * t17783;
    let t17794 = t2989 * t5398;
    let t17800 = t2987 * t5836;
    let t17804 = t2987 * t5842;
    let t17808 = t13847 * t4514;
    (t17764, t17770, t17784, t17794, t17800, t17804, t17808)
}
