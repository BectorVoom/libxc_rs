//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 858/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk858(t10: f64, t225: f64, t5902: f64, t670: f64, t2003: f64, t245: f64, t5926: f64, t1996: f64, t5931: f64, t1999: f64, t703: f64, t418: f64, t610: f64) -> (f64, f64, f64, f64, f64) {
    let t16553 = 0.43284165449459373508e0_f64 * t670 * t10 * t225 * t5902;
    let t16556 = 0.67090456446662028936e-1_f64 * t2003 * t245 * t5926;
    let t16557 = t1996 * t5931;
    let t16561 = 0.44726970964441352624e-1_f64 * t2003 * t703 * t1999;
    let t16562 = t418 * t610;
    (t16553, t16556, t16557, t16561, t16562)
}
