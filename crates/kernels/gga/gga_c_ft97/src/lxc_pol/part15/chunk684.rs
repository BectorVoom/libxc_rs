//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 684/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk684<F: Float>(t20087: F, t409: F, t64: F, t4474: F, t938: F, t8052: F, t11160: F, t15782: F, t15793: F, t15797: F, t1599: F, t1624: F, t19977: F, t19978: F, t19983: F, t19987: F, t19995: F, t19998: F, t20004: F, t20008: F, t20012: F, t20050: F, t3076: F, t3077: F, t372: F, t374: F, t4491: F, t534: F, t7906: F, t7914: F, t8042: F) -> (F, F) {
    let t20089 = t64 * t409 * t20087;
    let t20090 = t4474 * t938;
    let t20092 = t64 * t8052 * t20090;
    let t20097 = -F::new(0.32253953169881963531e-5) * t372 * t534 * t19978 - F::new(0.69764702839313376e-1) * t8042 * t19983 - F::new(0.69764702839313376e-2) * t1624 * t19987 - F::new(0.33776098467676728323e-5) * t534 * t19977 * t1599 + F::new(0.58097170218823199823e-3) * t372 * t19995 - F::new(0.58097170218823199822e-3) * t1624 * t19998 - F::new(0.279058811357253504e-2) * t372 * t7914 * t19978 + F::new(0.69764702839313376e-2) * t372 * t20004 + F::new(0.34882351419656688e-1) * t1624 * t20008 + F::new(0.34882351419656688e-1) * t1624 * t20012 - F::new(0.11619434043764639964e-3) * t372 * t7906 * t19978 - F::new(0.11627450473218896e-1) * t372 * t374 * t20050 - F::new(0.20279640676073749279e-3) * t15797 * t11160 - F::new(0.40559281352147498558e-4) * t7906 * t19977 * t1599 + F::new(0.20279640676073749279e-3) * t15793 * t15782 - t20089 - F::new(6.0) * t20092 + F::new(6.0) * t3076 * t3077 * t4491;
    (t20090, t20097)
}
