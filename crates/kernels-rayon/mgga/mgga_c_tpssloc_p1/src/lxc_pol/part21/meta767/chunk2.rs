//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2647/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647(t19530: f64, t626: f64, t12774: f64, t12795: f64, t12802: f64, t1447: f64, t16: f64, t19488: f64, t19489: f64, t19492: f64, t19499: f64, t19503: f64, t19504: f64, t19517: f64, t2219: f64, t2248: f64, t2336: f64, t2341: f64, t2351: f64, t2355: f64, t30171: f64, t30307: f64, t45697: f64, t45707: f64, t45751: f64, t45762: f64, t5469: f64, t5472: f64, t5475: f64, t657: f64, t659: f64, t92: f64) -> (f64, f64) {
    let t55420 = t626 * t19530;
    let t55457 = -50.0_f64 / 9.0_f64 * t657 * t19504 + 200.0_f64 / 27.0_f64 * t5475 * t2355 + 400.0_f64 / 81.0_f64 * t2336 * t5469 + 200.0_f64 / 27.0_f64 * t2336 * t5472 + 400.0_f64 / 81.0_f64 * t5475 * t2351 + 50.0_f64 / 9.0_f64 * t1447 * t12802 + 40.0_f64 / 27.0_f64 * t45707 * t30307 * t2219 - 40.0_f64 / 27.0_f64 * t45697 * t30171 * t2219 - 20.0_f64 / 3.0_f64 * t12774 * t19492 * t16 + 20.0_f64 / 3.0_f64 * t12795 * t19517 * t16 - t45751 + t45762 + 100.0_f64 / 81.0_f64 * t657 * t19489 - 10.0_f64 / 27.0_f64 * t92 * t19488 * t2248 - 100.0_f64 / 27.0_f64 * t657 * t19499 + 20.0_f64 / 9.0_f64 * t92 * t2341 * t19503 * t659;
    (t55420, t55457)
}
