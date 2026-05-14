//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 550/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk550<F: Float>(t62: F, t66: F, t401: F, t77: F, t408: F, t428: F, t3020: F, t1655: F, t1300: F, t1603: F, t1669: F, t1701: F, t1712: F, t3019: F, t372: F, t374: F, t385: F, t534: F, t7895: F, t79: F, t7900: F, t7906: F, t7914: F, t7919: F, t7926: F, t7930: F, t7936: F, t7939: F, t7978: F, t7982: F) -> (F, F, F, F, F, F, F, F) {
    let t7983 = t62 * t66;
    let t7984 = t77 * t401;
    let t7985 = t7983 * t7984;
    let t7988 = t408 * t428;
    let t7989 = t3020 * t7988;
    let t7992 = t77 * t1655;
    let t7993 = t3020 * t7992;
    let t7996 = -0.17782141943527538963e-1 * t1300 * t1701 * t7895 - 0.32253953169881963531e-5 * t372 * t534 * t7900 - 0.11619434043764639964e-3 * t372 * t7906 * t7900 - 0.279058811357253504e-2 * t372 * t7914 * t7900 - 0.69764702839313376e-1 * t7919 * t385 - 0.26701719421757626014e-2 * t79 * t7926 + 0.139529405678626752e-1 * t1603 * t7930 + 0.27529390119979671431e0 * t79 * t7936 + 12.0 * t1669 * t7939 * t1712 - 0.11627450473218896e-1 * t372 * t374 * t7978 + 0.40559281352147498558e-3 * t7982 * t7985 - 0.20279640676073749279e-3 * t7982 * t7989 + 0.20279640676073749279e-3 * t3019 * t7993;
    (t7983, t7984, t7985, t7988, t7989, t7992, t7993, t7996)
}
