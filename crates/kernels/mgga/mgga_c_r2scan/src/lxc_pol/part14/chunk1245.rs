//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1245/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1245<F: Float>(t322: F, t41891: F, t1020: F, t11249: F, t11276: F, t11278: F, t11280: F, t1129: F, t12285: F, t12286: F, t12288: F, t12302: F, t1312: F, t2410: F, t333: F, t335: F, t337: F, t3524: F, t3526: F, t3761: F, t839: F, t8438: F) -> (F, F) {
    let t332 = F::new(0.25e1) < t322;
    let t41978 = piecewise3::<f64>(t332, F::new(0.0), t41891);
    let t42003 = -F::new(0.1088826475632e2) * t3761 * t1312 + F::new(0.734774460522e2) * t11249 * t1020 - F::new(0.17408e1) * t839 * t12285 - F::new(0.8704e0) * t333 * t41978 - F::new(0.4607056813647e1) * t335 * t41978 + F::new(0.122462410087e2) * t337 * t41978 - F::new(0.7662840944824e2) * t12302 * t839 + F::new(0.3101306810232e2) * t12286 * t839 - F::new(0.4355305902528e1) * t12288 * t839 - F::new(0.9214113627294e1) * t11276 * t1020 - F::new(0.18428227254588e2) * t11278 * t1020 - F::new(0.18428227254588e2) * t3524 * t2410 - F::new(0.9214113627294e1) * t11280 * t1020 - F::new(0.18428227254588e2) * t3526 * t2410 - F::new(0.9214113627294e1) * t1129 * t8438;
    (t41978, t42003)
}
