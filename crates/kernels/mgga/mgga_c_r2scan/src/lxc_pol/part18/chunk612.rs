//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 612/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk612<F: Float>(t1081: F, t839: F, t333: F, t3386: F, t335: F, t337: F, t339: F, t341: F, t1083: F, t1085: F, t1087: F, t1089: F, t1091: F, t343: F) -> (F, F, F, F, F, F, F) {
    let t3388 = t839 * t1081;
    let t3390 = t333 * t3386;
    let t3394 = t335 * t3386;
    let t3398 = t337 * t3386;
    let t3402 = t339 * t3386;
    let t3406 = t341 * t3386;
    let t3412 = -F::new(0.64e0) * t3386 - F::new(0.8704e0) * t3388 - F::new(0.8704e0) * t3390 - F::new(0.9214113627294e1) * t1083 * t839 - F::new(0.4607056813647e1) * t3394 + F::new(0.367387230261e2) * t1085 * t839 + F::new(0.122462410087e2) * t3398 - F::new(0.3831420472412e2) * t1087 * t839 - F::new(0.957855118103e1) * t3402 + F::new(0.1550653405116e2) * t1089 * t839 + F::new(0.3101306810232e1) * t3406 - F::new(0.2177652951264e1) * t1091 * t839 - F::new(0.362942158544e0) * t343 * t3386;
    (t3388, t3390, t3394, t3398, t3402, t3406, t3412)
}
