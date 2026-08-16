//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2183/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2183(t1880: f64, t7488: f64, t82124: f64, t1911: f64, t40889: f64, t23185: f64, t25045: f64, t82074: f64, t254: f64, t799: f64, t225: f64, t25161: f64) -> (f64, f64, f64, f64, f64) {
    let t87746 = t1880 * t82124 * t7488;
    let t87748 = t40889 * t1911;
    let t87753 = t23185 * t82074 * t25045;
    let t87754 = 0.16449340668482264365e-1_f64 * t87753;
    let t87755 = t799 * t254;
    let t87758 = t25161 * t225;
    (t87746, t87748, t87754, t87755, t87758)
}
