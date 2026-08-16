//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 858/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk858<F: Float>(t10: F, t225: F, t5902: F, t670: F, t2003: F, t245: F, t5926: F, t1996: F, t5931: F, t1999: F, t703: F, t418: F, t610: F) -> (F, F, F, F, F) {
    let t16553 = F::cast_from(0.43284165449459373508e0_f64) * t670 * t10 * t225 * t5902;
    let t16556 = F::cast_from(0.67090456446662028936e-1_f64) * t2003 * t245 * t5926;
    let t16557 = t1996 * t5931;
    let t16561 = F::cast_from(0.44726970964441352624e-1_f64) * t2003 * t703 * t1999;
    let t16562 = t418 * t610;
    (t16553, t16556, t16557, t16561, t16562)
}
