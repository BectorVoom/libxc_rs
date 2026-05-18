//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 689/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk689<F: Float>(t5548: F, t5550: F, t587: F, t1868: F, t579: F, t1672: F, t563: F, t561: F, t1: F, t1952: F, t119: F, t713: F) -> (F, F, F, F, F, F, F) {
    let t5551 = t5548 * t5550;
    let t5553 = F::new(8.0) / F::new(15.0) * t587 * t5551;
    let t5555 = F::new(2.0) / F::new(5.0) * t579 * t1868;
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5558 = F::new(8.0) / F::new(45.0) * t5557;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    (t5551, t5553, t5555, t5556, t5558, t5559, t5560)
}
