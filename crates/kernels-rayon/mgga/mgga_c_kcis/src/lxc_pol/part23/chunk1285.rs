//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1285/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1285(t27563: f64, t28853: f64, t1598: f64, t251: f64, t54624: f64, t1607: f64, t613: f64, t17980: f64, t27615: f64, t18256: f64, t27567: f64, t27583: f64, t27598: f64, t28805: f64, t3801: f64, t4440: f64, t7981: f64, t94223: f64, t94225: f64, t98014: f64, t98022: f64, t98030: f64, t98036: f64, t98039: f64) -> (f64, f64, f64, f64) {
    let t98988 = 0.82448622685185185186e-4_f64 * t28853 * t27563;
    let t98994 = t54624 * t251 * t1598;
    let t99002 = t613 * t1607;
    let t99004 = t99002 * t17980 * t27615;
    let t99013 = t18256 * t251 * t1598;
    let t99016 = -0.23214722222222222222e-2_f64 * t98014 - 0.92835860883789062501e-5_f64 * t98994 * t27598 - 0.61905925925925925924e-2_f64 * t98022 - 0.11607361111111111111e-2_f64 * t98030 - 0.15476481481481481481e-2_f64 * t94223 + 0.10317654320987654321e-2_f64 * t94225 + 0.25794135802469135802e-2_f64 * t98036 + 0.2782641015625e-3_f64 * t27567 * t99004 - 0.15476481481481481481e-2_f64 * t98039 - 0.23168402777777777778e-3_f64 * t27583 * t4440 * t28805 * t3801 - 0.23168402777777777778e-3_f64 * t99013 * t7981;
    (t98988, t99004, t99013, t99016)
}
