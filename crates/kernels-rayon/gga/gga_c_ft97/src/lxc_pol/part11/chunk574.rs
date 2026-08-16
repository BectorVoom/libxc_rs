//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 574/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk574(t62: f64, t66: f64, t401: f64, t77: f64, t408: f64, t428: f64, t3020: f64, t1655: f64, t1300: f64, t1603: f64, t1669: f64, t1701: f64, t1712: f64, t3019: f64, t372: f64, t374: f64, t385: f64, t534: f64, t7895: f64, t79: f64, t7900: f64, t7906: f64, t7914: f64, t7919: f64, t7926: f64, t7930: f64, t7936: f64, t7939: f64, t7978: f64, t7982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7983 = t62 * t66;
    let t7984 = t77 * t401;
    let t7985 = t7983 * t7984;
    let t7988 = t408 * t428;
    let t7989 = t3020 * t7988;
    let t7992 = t77 * t1655;
    let t7993 = t3020 * t7992;
    let t7996 = -0.17782141943527538963e-1_f64 * t1300 * t1701 * t7895 - 0.32253953169881963531e-5_f64 * t372 * t534 * t7900 - 0.11619434043764639964e-3_f64 * t372 * t7906 * t7900 - 0.279058811357253504e-2_f64 * t372 * t7914 * t7900 - 0.69764702839313376e-1_f64 * t7919 * t385 - 0.26701719421757626014e-2_f64 * t79 * t7926 + 0.139529405678626752e-1_f64 * t1603 * t7930 + 0.27529390119979671431e0_f64 * t79 * t7936 + 12.0_f64 * t1669 * t7939 * t1712 - 0.11627450473218896e-1_f64 * t372 * t374 * t7978 + 0.40559281352147498558e-3_f64 * t7982 * t7985 - 0.20279640676073749279e-3_f64 * t7982 * t7989 + 0.20279640676073749279e-3_f64 * t3019 * t7993;
    (t7983, t7984, t7985, t7988, t7989, t7992, t7993, t7996)
}
