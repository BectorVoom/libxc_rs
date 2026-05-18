//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 792/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk792<F: Float>(t366: F, t6553: F, t899: F, t6242: F, t904: F, t916: F, t2209: F, t825: F, t2182: F, t337: F, t5: F, t2146: F) -> (F, F, F, F, F, F) {
    let t6555 = t899 * t6553 * t366;
    let t6557 = t916 * t904 * t6242;
    let t6560 = t825 * t2209;
    let t6562 = t337 * t5 * t2182;
    let t6563 = t6560 * t6562;
    let t6565 = F::new(3.0) / F::new(16.0) * t2146 * t6563;
    (t6555, t6557, t6560, t6562, t6563, t6565)
}
