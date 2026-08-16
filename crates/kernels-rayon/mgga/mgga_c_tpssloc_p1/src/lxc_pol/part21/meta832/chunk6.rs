//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2938/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938(t135: f64, t17843: f64, t973: f64, t13831: f64, t17804: f64, t2986: f64, t2988: f64, t340: f64, t343: f64, t42811: f64, t42817: f64, t42873: f64, t42877: f64, t42893: f64, t42895: f64, t4531: f64, t47887: f64, t47938: f64, t61103: f64, t61124: f64, t61138: f64, t61150: f64, t61163: f64, t974: f64) -> f64 {
    let t61172 = t973 * t135 * t17843;
    let t61181 = 0.22222222222222222222e-2_f64 * t2986 * t2988 * t61103 - 0.33333333333333333333e-2_f64 * t2986 * t4531 * t47887 + 0.74074074074074074072e-3_f64 * t47938 - 0.16460905349794238683e-2_f64 * t42811 - t42817 - 0.83333333333333333332e-3_f64 * t973 * t974 * t340 * (t61124 + t61138 + t61150 + t61163) * t343 - 0.55555555555555555554e-3_f64 * t61172 - 0.6172839506172839506e-4_f64 * t42873 - 0.82304526748971193413e-4_f64 * t42877 + 0.20576131687242798354e-3_f64 * t42893 - 0.18106995884773662551e-2_f64 * t42895 - 0.55555555555555555554e-3_f64 * t2986 * t17804 * t13831;
    t61181
}
