//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3266/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3266<F: Float>(t235: F, t239: F, t2476: F, t246: F, t4365: F, t10770: F, t14802: F, t14917: F, t18444: F, t23334: F, t2745: F, t40753: F, t40759: F, t40761: F, t40765: F, t40771: F, t4504: F, t50791: F, t50933: F, t50937: F, t50939: F, t50941: F) -> (F, F, F, F) {
    let t61999 = t235 * t239;
    let t62000 = t61999 * t2476;
    let t62002 = t246 * t4365;
    let t62008 = -F::cast_from(0.42874018118069736972e-2_f64) * t2745 * t10770 * t18444 * t14917 - F::cast_from(0.45178982497454656791e-5_f64) * t40753 - t40759 - F::cast_from(0.45178982497454656791e-5_f64) * t40761 + F::cast_from(0.16264433699083676444e-3_f64) * t40765 + t40771 + F::cast_from(0.10164000561857065645e-2_f64) * t50791 + F::cast_from(0.50820002809285328225e-3_f64) * t50933 + F::cast_from(0.14291339372689912324e-4_f64) * t50937 + F::cast_from(0.1219527626469539185e-2_f64) * t50939 + F::cast_from(0.34299214494455789578e-1_f64) * t4504 * t62000 * t62002 * t23334 * t14802 + F::new(455.0) / F::new(324.0) * t50941;
    (t61999, t62000, t62002, t62008)
}
