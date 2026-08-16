//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1430/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1430(t3395: f64, t1124: f64, t11349: f64, t3355: f64, t427: f64, t3358: f64, t11176: f64, t1147: f64, t3368: f64, t3400: f64, t11285: f64, t11300: f64, t11307: f64, t11353: f64, t11356: f64, t11361: f64, t11365: f64, t1137: f64, t11400: f64, t11415: f64, t11420: f64, t1156: f64, t1157: f64, t3332: f64, t3357: f64, t3359: f64, t3371: f64, t3396: f64, t3401: f64, t3403: f64, t3404: f64, t43679: f64, t44142: f64, t44146: f64, t44155: f64, t44161: f64, t44164: f64, t44167: f64) -> (f64, f64) {
    let t44168 = t3395 * t3395;
    let t44172 = t1124 * t11349;
    let t44175 = t3355 * t3355;
    let t44177 = t427 / t44175;
    let t44178 = t3358 * t3358;
    let t44179 = 1.0_f64 / t44178;
    let t44183 = t11176 * t1147;
    let t44188 = t3368 * t3400;
    let t44198 = 24.0_f64 * t11415 * t11307 - 24.0_f64 * t11420 * t44142 * t1137 - 6.0_f64 * t3332 * t44146 * t1137 + 0.96491876992155210402e2_f64 * t3357 * t44146 * t3359 - 0.12304822629859687989e5_f64 * t44155 * t43679 * t11285 + t44161 + t44164 - t44167 + 0.51947577317044391277e2_f64 * t3401 * t44168 * t3403 + 0.82761620670837440481e4_f64 * t44172 * t11353 + 0.19964560303604640732e6_f64 * t44177 * t44142 * t44179 + 0.23392894490538584828e1_f64 * t44183 * t1157 + 0.35089341735807877242e1_f64 * t11356 * t3396 + 0.10389515463408878255e3_f64 * t44188 * t3404 + 0.23392894490538584828e1_f64 * t3371 * t11400 + 0.14035736694323150897e2_f64 * t11361 * t11300 - 0.14035736694323150897e2_f64 * t11365 * t43679 * t1156;
    (t44168, t44198)
}
