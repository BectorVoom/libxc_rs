//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 526/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk526<F: Float>(t4292: F, t4294: F, t3954: F, t584: F, t583: F, t1546: F, t556: F, t4136: F, t578: F, t3722: F, t555: F, t4246: F, t4250: F, t4252: F, t4258: F, t4263: F, t4267: F, t4271: F, t4275: F, t4279: F, t4282: F, t4284: F, t4289: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4295 = t4292 * t4294;
    let t4297 = t584 * t3954;
    let t4298 = t583 * t4297;
    let t4299 = t1546 * t4298;
    let t4301 = F::cast_from(1.0_f64) / t556;
    let t4302 = t4301 * t4136;
    let t4303 = t583 * t4302;
    let t4304 = t578 * t4303;
    let t4306 = t555 * t3722;
    let t4307 = t583 * t4306;
    let t4308 = t578 * t4307;
    let t4310 = t4246 / F::cast_from(16.0_f64) - t4250 / F::cast_from(8.0_f64) + t4252 / F::cast_from(12.0_f64) + t4258 / F::cast_from(8.0_f64) - t4263 / F::cast_from(12.0_f64) - t4267 / F::cast_from(16.0_f64) - t4271 / F::cast_from(72.0_f64) + t4275 / F::cast_from(24.0_f64) - t4279 / F::cast_from(256.0_f64) + t4282 / F::cast_from(128.0_f64) - t4284 / F::cast_from(96.0_f64) - t4289 / F::cast_from(128.0_f64) + t4295 / F::cast_from(96.0_f64) + t4299 / F::cast_from(256.0_f64) - t4304 / F::cast_from(576.0_f64) - t4308 / F::cast_from(192.0_f64);
    (t4295, t4297, t4298, t4299, t4301, t4302, t4303, t4304, t4306, t4307, t4308, t4310)
}
