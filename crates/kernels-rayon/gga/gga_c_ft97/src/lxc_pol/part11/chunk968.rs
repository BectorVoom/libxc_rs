//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 968/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk968(t549: f64, t554: f64, t8153: f64, t8157: f64, t1355: f64, t1995: f64, t2001: f64, t2002: f64, t2059: f64, t2071: f64, t3392: f64, t39824: f64, t39828: f64, t399: f64, t40093: f64, t539: f64, t555: f64, t5802: f64, t5818: f64, t8807: f64, t8812: f64, t8865: f64, t8877: f64, t8885: f64, t8894: f64, t8907: f64, t8932: f64) -> f64 {
    let t40150 = t549 * t8153 * t8157 * t554;
    let t40164 = 0.22445349300913785316e3_f64 * t5802 * t39824 - 0.11222674650456892658e3_f64 * t1355 * t39828 - 36.0_f64 * t3392 * t8907 * t2059 * t2071 + 8.0_f64 * t3392 * t8877 * t8932 - 8.0_f64 * t2001 * t2002 * t8932 - 12.0_f64 * t2001 * t8865 * t2071 + 0.13035593825592482769e1_f64 * t5818 * t40150 - 0.43451979418641609231e0_f64 * t3392 * t40150 - 0.11093760908123778558e3_f64 * t8812 * t8807 * t539 + 0.14498192132169191472e2_f64 * t1995 * t40093 * t8885 + 0.14498192132169191472e2_f64 * t8894 * t555 * t399;
    t40164
}
