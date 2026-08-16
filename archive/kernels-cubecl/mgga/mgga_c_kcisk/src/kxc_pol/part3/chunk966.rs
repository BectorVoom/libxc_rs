//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 966/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk966<F: Float>(t14270: F, t415: F, t1327: F, t3924: F, t4158: F, t1220: F, t13125: F, t13441: F, t14242: F, t14247: F, t14250: F, t14253: F, t14258: F, t14262: F, t14268: F, t3925: F, t3930: F, t412: F) -> (F, F) {
    let t14271 = t415 * t14270;
    let t14273 = t1327 * t3924;
    let t14274 = t14273 * t4158;
    let t14279 = -F::cast_from(0.223494e0_f64) * t3930 * t13441 + F::cast_from(0.223494e0_f64) * t14242 * t3925 + F::cast_from(0.48640370370370370369e-1_f64) * t14247 + t13125 * t412 + F::cast_from(0.44218518518518518518e-2_f64) * t14250 + F::cast_from(0.72960555555555555553e-1_f64) * t14253 + F::cast_from(0.55273148148148148145e-2_f64) * t14258 - F::cast_from(0.11054629629629629629e-2_f64) * t14262 + F::cast_from(0.99491666666666666664e-2_f64) * t14268 - F::cast_from(0.19898333333333333333e-1_f64) * t14271 + F::cast_from(0.223494e0_f64) * t3930 * t14274 + F::cast_from(0.579e0_f64) * t1220 * t14274;
    (t14271, t14279)
}
