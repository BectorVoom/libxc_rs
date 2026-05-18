//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1246/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1246<F: Float>(t1020: F, t1127: F, t11274: F, t11282: F, t1129: F, t1131: F, t12298: F, t1310: F, t1312: F, t2410: F, t2412: F, t339: F, t341: F, t343: F, t3522: F, t3526: F, t3530: F, t3745: F, t3749: F, t41978: F, t839: F, t8438: F, t8465: F) -> F {
    let t42035 = -F::new(0.18428227254588e2) * t12298 * t839 - F::new(0.9214113627294e1) * t3749 * t1310 + F::new(0.367387230261e2) * t11282 * t1020 + F::new(0.734774460522e2) * t3530 * t2410 + F::new(0.367387230261e2) * t1131 * t8438 - F::new(0.957855118103e1) * t339 * t41978 + F::new(0.3101306810232e1) * t341 * t41978 - F::new(0.362942158544e0) * t343 * t41978 - F::new(0.9214113627294e1) * t1312 * t3745 - F::new(0.8704e0) * t8438 * t1127 - F::new(0.17408e1) * t2410 * t3522 - F::new(0.8704e0) * t1020 * t11274 - F::new(0.8704e0) * t1310 * t3745 + F::new(0.1469548921044e3) * t3526 * t2412 + F::new(0.1469548921044e3) * t1129 * t8465;
    t42035
}
