//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 747/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk747<F: Float>(t609: F, t1610: F, t4456: F, t286: F, t4390: F, t1608: F, t1599: F, t4424: F, t4427: F, t4430: F, t4435: F, t4439: F, t4442: F, t4447: F, t4451: F) -> (F, F, F, F, F, F, F) {
    let t614 = F::cast_from(0.0_f64) < t609;
    let t4457 = t1610 * t1610;
    let t4458 = t4456 * t4457;
    let t4459 = t286 * t4458;
    let t4463 = piecewise3::<F>(t614, t4390, -t4390);
    let t4464 = t1608 * t4463;
    let t4465 = t286 * t4464;
    let t4468 = -t4424 + t4427 / F::cast_from(864.0_f64) - t4430 / F::cast_from(288.0_f64) + t1599 * t4435 / F::cast_from(432.0_f64) - t4439 * t4442 / F::cast_from(288.0_f64) - t1599 * t4447 / F::cast_from(288.0_f64) + t1599 * t4451 / F::cast_from(576.0_f64) + t1599 * t4459 / F::cast_from(96.0_f64) - t1599 * t4465 / F::cast_from(192.0_f64);
    (t4457, t4458, t4459, t4463, t4464, t4465, t4468)
}
