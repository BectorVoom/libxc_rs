//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3266/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266(t235: f64, t239: f64, t2476: f64, t246: f64, t4365: f64, t10770: f64, t14802: f64, t14917: f64, t18444: f64, t23334: f64, t2745: f64, t40753: f64, t40759: f64, t40761: f64, t40765: f64, t40771: f64, t4504: f64, t50791: f64, t50933: f64, t50937: f64, t50939: f64, t50941: f64) -> (f64, f64, f64, f64) {
    let t61999 = t235 * t239;
    let t62000 = t61999 * t2476;
    let t62002 = t246 * t4365;
    let t62008 = -0.42874018118069736972e-2_f64 * t2745 * t10770 * t18444 * t14917 - 0.45178982497454656791e-5_f64 * t40753 - t40759 - 0.45178982497454656791e-5_f64 * t40761 + 0.16264433699083676444e-3_f64 * t40765 + t40771 + 0.10164000561857065645e-2_f64 * t50791 + 0.50820002809285328225e-3_f64 * t50933 + 0.14291339372689912324e-4_f64 * t50937 + 0.1219527626469539185e-2_f64 * t50939 + 0.34299214494455789578e-1_f64 * t4504 * t62000 * t62002 * t23334 * t14802 + 455.0_f64 / 324.0_f64 * t50941;
    (t61999, t62000, t62002, t62008)
}
