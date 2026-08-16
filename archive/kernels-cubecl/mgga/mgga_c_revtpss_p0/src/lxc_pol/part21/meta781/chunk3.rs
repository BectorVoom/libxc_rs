//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2796/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2796<F: Float>(t2801: F, t51421: F, t10073: F, t14588: F, t10542: F, t14563: F, t14502: F, t14507: F, t14546: F, t14547: F, t2646: F, t2724: F, t39683: F, t39685: F, t39687: F, t4504: F, t4514: F, t51408: F, t51418: F) -> F {
    let t51422 = t51421 * t2801;
    let t51424 = t10073 * t14588;
    let t51429 = t10542 * t14563;
    let t51430 = F::cast_from(0.39029762157531132076e-1_f64) * t51429;
    let t51431 = F::cast_from(0.69394917116090352834e-2_f64) * t39683 - F::cast_from(0.30356481678079769392e-1_f64) * t51408 - F::cast_from(0.29272321618148349057e-1_f64) * t39685 - F::cast_from(0.11853808529283920877e2_f64) * t14546 * t14502 * t14547 + F::cast_from(0.11853808529283920877e2_f64) * t4504 * t14502 * t2724 + F::cast_from(0.21951497276451705329e-1_f64) * t39687 - F::cast_from(0.58544643236296698113e-1_f64) * t51418 - F::cast_from(0.58544643236296698113e-1_f64) * t51422 - F::cast_from(0.39029762157531132075e-2_f64) * t51424 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t14507 * t2646 + t51430;
    t51431
}
