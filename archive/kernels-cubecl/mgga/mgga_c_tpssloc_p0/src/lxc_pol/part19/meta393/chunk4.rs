//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1495/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1495<F: Float>(t45405: F, t45545: F, t112: F, t12512: F, t111: F, t3931: F, t12521: F, t12524: F, t12529: F, t12532: F, t1395: F, t1401: F, t16535: F, t2319: F, t2363: F, t39231: F, t3938: F, t3941: F, t45510: F, t577: F, t671: F, t9416: F) -> (F, F) {
    let t45546 = t45405 + t45545;
    let t45557 = t12512 * t112;
    let t45560 = t3931 * t111;
    let t45580 = F::cast_from(0.45e1_f64) * t45546 * t577 + F::cast_from(54.0_f64) * t45557 * t671 + F::cast_from(162.0_f64) * t45560 * t2319 + F::cast_from(81.0_f64) * t12521 * t2363 + F::cast_from(108.0_f64) * t1395 * t12529 + F::cast_from(324.0_f64) * t12524 * t12532 + F::cast_from(54.0_f64) * t3938 * t9416 + F::cast_from(162.0_f64) * t16535 * t2363 + F::cast_from(81.0_f64) * t3941 * t39231 + F::cast_from(108.0_f64) * t3941 * t671 * t9416 + F::cast_from(0.135e2_f64) * t1401 * t45510;
    (t45546, t45580)
}
