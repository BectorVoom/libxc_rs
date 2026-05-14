//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 572/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk572<F: Float>(t1020: F, t839: F, t2410: F, t333: F, t335: F, t337: F, t339: F, t341: F, t1022: F, t1024: F, t1026: F, t1028: F, t1030: F, t343: F, t1035: F, t1338: F) -> (F, F, F, F, F, F, F, F) {
    let t2412 = t839 * t1020;
    let t2414 = t333 * t2410;
    let t2418 = t335 * t2410;
    let t2422 = t337 * t2410;
    let t2426 = t339 * t2410;
    let t2430 = t341 * t2410;
    let t2436 = -0.64e0 * t2410 - 0.8704e0 * t2412 - 0.8704e0 * t2414 - 0.9214113627294e1 * t1022 * t839 - 0.4607056813647e1 * t2418 + 0.367387230261e2 * t1024 * t839 + 0.122462410087e2 * t2422 - 0.3831420472412e2 * t1026 * t839 - 0.957855118103e1 * t2426 + 0.1550653405116e2 * t1028 * t839 + 0.3101306810232e1 * t2430 - 0.2177652951264e1 * t1030 * t839 - 0.362942158544e0 * t343 * t2410;
    let t2437 = t1338 * t1035;
    (t2412, t2414, t2418, t2422, t2426, t2430, t2436, t2437)
}
