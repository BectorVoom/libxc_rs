//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 916/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk916(t776: f64, t865: f64, t254: f64, t799: f64, t10143: f64, t1081: f64, t28: f64, t40772: f64, t6514: f64, t671: f64, t1868: f64, t2363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t87036 = t776 * t865;
    let t87755 = t799 * t254;
    let t89849 = t10143 * t1081;
    let t89953 = t40772 * t28;
    let t90041 = t6514 * t671;
    let t90044 = t1868 * t2363;
    (t87036, t87755, t89849, t89953, t90041, t90044)
}
