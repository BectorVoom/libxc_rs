//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 349/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk349<F: Float>(t1312: F, t333: F, t335: F, t337: F, t339: F, t1310: F, t341: F, t343: F, t349: F, t854: F) -> (F, F, F) {
    let t1316 = t333 * t1312;
    let t1320 = t335 * t1312;
    let t1324 = t337 * t1312;
    let t1328 = t339 * t1312;
    let t1336 = -F::cast_from(0.64e0_f64) * t1310 - F::cast_from(0.8704e0_f64) * t1312 - F::cast_from(0.8704e0_f64) * t333 * t1310 - F::cast_from(0.9214113627294e1_f64) * t1316 - F::cast_from(0.4607056813647e1_f64) * t335 * t1310 + F::cast_from(0.367387230261e2_f64) * t1320 + F::cast_from(0.122462410087e2_f64) * t337 * t1310 - F::cast_from(0.3831420472412e2_f64) * t1324 - F::cast_from(0.957855118103e1_f64) * t339 * t1310 + F::cast_from(0.1550653405116e2_f64) * t1328 + F::cast_from(0.3101306810232e1_f64) * t341 * t1310 - F::cast_from(0.2177652951264e1_f64) * t341 * t1312 - F::cast_from(0.362942158544e0_f64) * t343 * t1310;
    let t1337 = t854 * t349;
    let t1338 = F::cast_from(1.0_f64) / t1337;
    (t1336, t1337, t1338)
}
