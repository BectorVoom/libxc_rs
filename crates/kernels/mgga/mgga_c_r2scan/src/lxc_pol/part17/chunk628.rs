//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 628/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk628<F: Float>(t1127: F, t839: F, t333: F, t3522: F, t335: F, t337: F, t339: F, t341: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t3524 = t839 * t1127;
    let t3526 = t333 * t3522;
    let t3530 = t335 * t3522;
    let t3534 = t337 * t3522;
    let t3538 = t339 * t3522;
    let t3542 = t341 * t3522;
    let t3548 = -F::new(0.64e0) * t3522 - F::new(0.8704e0) * t3524 - F::new(0.8704e0) * t3526 - F::new(0.9214113627294e1) * t1129 * t839 - F::new(0.4607056813647e1) * t3530 + F::new(0.367387230261e2) * t1131 * t839 + F::new(0.122462410087e2) * t3534 - F::new(0.3831420472412e2) * t1133 * t839 - F::new(0.957855118103e1) * t3538 + F::new(0.1550653405116e2) * t1135 * t839 + F::new(0.3101306810232e1) * t3542 - F::new(0.2177652951264e1) * t1137 * t839 - F::new(0.362942158544e0) * t343 * t3522;
    (t3524, t3526, t3530, t3534, t3538, t3542, t3548)
}
