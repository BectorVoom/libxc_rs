//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 849/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk849<F: Float>(t1533: F, t510: F, t5651: F, t1590: F, t2030: F, t2032: F, t1592: F, t475: F, t1952: F, t4579: F, t553: F, t1971: F, t4585: F, t5697: F) -> (F, F, F, F, F) {
    let t16428 = t5651 * t510 * t1533;
    let t16431 = t2030 * t1590;
    let t16432 = t16431 * t2032;
    let t16436 = t475 * t1592 * t2030;
    let t16441 = F::new(0.39507780657818961764e-1) * t1952 * t4579 * t553;
    let t16444 = F::new(0.13871971944573393855e-1) * t5697 * t4585 * t1971;
    (t16428, t16432, t16436, t16441, t16444)
}
