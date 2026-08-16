//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1047/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1047(t30889: f64, t345: f64, t1022: f64, t8391: f64, t1060: f64, t30843: f64, t383: f64, t1003: f64, t1058: f64, t1920: f64, t30876: f64, t30879: f64, t30882: f64, t30886: f64, t353: f64, t6680: f64, t6687: f64, t6797: f64, t8401: f64, t8404: f64) -> (f64, f64, f64) {
    let t30890 = t345 * t30889;
    let t30894 = t8391 * t1022;
    let t30895 = t30894 * t1060;
    let t30897 = t383 * t30843;
    let t30899 = -0.43864908449286038307e-1_f64 * t6680 * t8401 + t30876 + 0.54831135561607547883e-2_f64 * t6687 * t30879 - 0.16449340668482264365e-1_f64 * t6687 * t30882 + 0.16449340668482264365e-1_f64 * t6797 * t30886 + 0.16449340668482264365e-1_f64 * t1920 * t30890 + t1003 * t8404 + t1058 * t30895 + t353 * t30897;
    (t30895, t30897, t30899)
}
