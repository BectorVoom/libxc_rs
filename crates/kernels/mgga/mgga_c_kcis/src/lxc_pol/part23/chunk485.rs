//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 485/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk485<F: Float>(t2642: F, t518: F, t1409: F, t833: F, t1419: F, t2645: F, t517: F) -> (F, F, F, F, F) {
    let t4024 = t518 * t2642;
    let t4027 = t1409 * t833;
    let t4028 = t4027 * t1419;
    let t4031 = t518 * t2645;
    let t4034 = t517 * t517;
    let t4035 = F::new(1.0) / t4034;
    (t4024, t4028, t4031, t4034, t4035)
}
