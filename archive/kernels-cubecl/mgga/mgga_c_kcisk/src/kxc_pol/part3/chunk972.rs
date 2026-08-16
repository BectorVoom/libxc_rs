//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 972/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk972<F: Float>(t14340: F, t1506: F, t14302: F, t14305: F, t14308: F, t14310: F, t14313: F, t14316: F, t14318: F, t14322: F, t14324: F, t14326: F, t14328: F, t14331: F, t14335: F, t14338: F) -> (F, F) {
    let t14341 = t14340 * t1506;
    let t14343 = t14302 / F::cast_from(192.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t14305 - t14308 / F::cast_from(8.0_f64) - t14310 / F::cast_from(8.0_f64) + t14313 / F::cast_from(64.0_f64) - t14316 + t14318 / F::cast_from(12.0_f64) + F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t14322 + t14324 / F::cast_from(6.0_f64) - t14326 / F::cast_from(24.0_f64) - F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t14328 - t14331 / F::cast_from(16.0_f64) - t14335 / F::cast_from(16.0_f64) + t14338 / F::cast_from(8.0_f64) + F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t14341;
    (t14341, t14343)
}
