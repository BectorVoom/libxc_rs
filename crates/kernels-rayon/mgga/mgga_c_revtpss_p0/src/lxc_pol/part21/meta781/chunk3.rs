//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2796/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796(t2801: f64, t51421: f64, t10073: f64, t14588: f64, t10542: f64, t14563: f64, t14502: f64, t14507: f64, t14546: f64, t14547: f64, t2646: f64, t2724: f64, t39683: f64, t39685: f64, t39687: f64, t4504: f64, t4514: f64, t51408: f64, t51418: f64) -> f64 {
    let t51422 = t51421 * t2801;
    let t51424 = t10073 * t14588;
    let t51429 = t10542 * t14563;
    let t51430 = 0.39029762157531132076e-1_f64 * t51429;
    let t51431 = 0.69394917116090352834e-2_f64 * t39683 - 0.30356481678079769392e-1_f64 * t51408 - 0.29272321618148349057e-1_f64 * t39685 - 0.11853808529283920877e2_f64 * t14546 * t14502 * t14547 + 0.11853808529283920877e2_f64 * t4504 * t14502 * t2724 + 0.21951497276451705329e-1_f64 * t39687 - 0.58544643236296698113e-1_f64 * t51418 - 0.58544643236296698113e-1_f64 * t51422 - 0.39029762157531132075e-2_f64 * t51424 - 0.19756347548806534796e1_f64 * t4514 * t14507 * t2646 + t51430;
    t51431
}
