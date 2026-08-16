//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1592/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1592(t4889: f64, t4916: f64, t1653: f64, t7319: f64, t4919: f64, t15293: f64, t4928: f64, t8034: f64, t4934: f64, t1184: f64, t460: f64, t6144: f64) -> (f64, f64, f64, f64, f64) {
    let t18536 = t4889 * t4916;
    let t18542 = t7319 * t1653;
    let t18543 = t4919 * t18542;
    let t18546 = t4919 * t15293;
    let t18549 = t8034 * t4928;
    let t18550 = t4934 * t18549;
    let t18554 = t6144 * t1184 * t460;
    (t18536, t18543, t18546, t18550, t18554)
}
