//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 959/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk959<F: Float>(t1820: F, t2559: F, t4352: F, t4957: F, t562: F, t1775: F, t1806: F, t617: F, t661: F, t1620: F, t1724: F, t7216: F) -> (F, F, F) {
    let t17783 = F::new(64.0) / F::new(9.0) * t1820 * t2559 * t562 * t4957 * t4352;
    let t17785 = F::new(8.0) / F::new(5.0) * t1775 * t1806;
    let t17786 = t661 * t617;
    let t17790 = F::new(32.0) / F::new(5.0) * t1620 * t7216 * t17786 * t1724;
    (t17783, t17785, t17790)
}
