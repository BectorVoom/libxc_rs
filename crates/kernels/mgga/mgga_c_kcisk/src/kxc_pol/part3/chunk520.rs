//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 520/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk520<F: Float>(t4209: F, t4211: F, t1413: F, t1481: F, t1489: F, t1501: F, t1513: F, t1517: F, t4176: F, t4183: F, t4186: F, t4190: F, t4194: F, t4198: F, t4201: F, t4206: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t4212 = t4209 * t4211;
    let t4214 = t1481 * t1413;
    let t4215 = t4214 * sigma0;
    let t4216 = t4215 * t1489;
    let t4218 = t1501 * t1513;
    let t4220 = t1501 * t1517;
    let t4222 = t4176 / F::cast_from(24.0_f64) - F::cast_from(19.0_f64) / F::cast_from(144.0_f64) * t4183 + t4186 / F::cast_from(18.0_f64) + t4190 / F::cast_from(256.0_f64) - t4194 / F::cast_from(192.0_f64) - t4198 / F::cast_from(16.0_f64) + t4201 / F::cast_from(3.0_f64) - t4206 / F::cast_from(12.0_f64) + t4212 / F::cast_from(8.0_f64) - t4216 / F::cast_from(8.0_f64) + t4218 / F::cast_from(24.0_f64) - t4220 / F::cast_from(96.0_f64);
    (t4212, t4214, t4215, t4216, t4218, t4220, t4222)
}
