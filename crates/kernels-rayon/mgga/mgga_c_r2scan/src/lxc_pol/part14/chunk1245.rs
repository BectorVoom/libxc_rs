//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1245/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1245(t322: f64, t41891: f64, t1020: f64, t11249: f64, t11276: f64, t11278: f64, t11280: f64, t1129: f64, t12285: f64, t12286: f64, t12288: f64, t12302: f64, t1312: f64, t2410: f64, t333: f64, t335: f64, t337: f64, t3524: f64, t3526: f64, t3761: f64, t839: f64, t8438: f64) -> (f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t41978 = piecewise3(t332, 0.0_f64, t41891);
    let t42003 = -0.1088826475632e2_f64 * t3761 * t1312 + 0.734774460522e2_f64 * t11249 * t1020 - 0.17408e1_f64 * t839 * t12285 - 0.8704e0_f64 * t333 * t41978 - 0.4607056813647e1_f64 * t335 * t41978 + 0.122462410087e2_f64 * t337 * t41978 - 0.7662840944824e2_f64 * t12302 * t839 + 0.3101306810232e2_f64 * t12286 * t839 - 0.4355305902528e1_f64 * t12288 * t839 - 0.9214113627294e1_f64 * t11276 * t1020 - 0.18428227254588e2_f64 * t11278 * t1020 - 0.18428227254588e2_f64 * t3524 * t2410 - 0.9214113627294e1_f64 * t11280 * t1020 - 0.18428227254588e2_f64 * t3526 * t2410 - 0.9214113627294e1_f64 * t1129 * t8438;
    (t41978, t42003)
}
