//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1302/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1302(t111916: f64, t111961: f64, t112006: f64, t112049: f64, t111845: f64, t12524: f64, t1458: f64, t16521: f64, t16524: f64, t20173: f64, t20176: f64, t20181: f64, t2199: f64, t30109: f64, t30112: f64, t30315: f64, t30385: f64, t30390: f64, t30534: f64, t30608: f64, t30611: f64, t33185: f64, t3938: f64, t3941: f64, t4072: f64, t5493: f64, t55353: f64, t55388: f64, t577: f64, t66958: f64, t671: f64, t8212: f64, t8273: f64, t8294: f64) -> (f64, f64) {
    let t112051 = t111916 + t111961 + t112006 + t112049;
    let t112062 = 54.0_f64 * t30112 * t20176 + 0.135e2_f64 * t111845 * t671 + 0.135e2_f64 * t66958 * t2199 + 0.135e2_f64 * t30109 * t5493 + 54.0_f64 * t55353 * t8294 + 54.0_f64 * t20173 * t30608 + 54.0_f64 * t3941 * t30315 * t1458 + 54.0_f64 * t3941 * t8273 * t4072 + 27.0_f64 * t16521 * t8273 + 0.135e2_f64 * t3938 * t30534 + 54.0_f64 * t12524 * t30608 + 27.0_f64 * t55388 * t8212 + 27.0_f64 * t3941 * t30534 * t671 + 0.45e1_f64 * t112051 * t577 + 27.0_f64 * t12524 * t30611 + 27.0_f64 * t30112 * t20181 + 54.0_f64 * t16524 * t30390 + 54.0_f64 * t33185 * t30385;
    (t112051, t112062)
}
