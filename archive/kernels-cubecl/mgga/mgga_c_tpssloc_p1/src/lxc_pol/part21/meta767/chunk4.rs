//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2649/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2649<F: Float>(t4067: F, t2331: F, t45421: F, t45422: F, t45424: F, t45426: F, t45656: F, t45658: F, t45660: F, t45662: F, t45688: F, t45690: F, t55420: F, t55457: F, t55512: F, t64: F, t656: F) -> F {
    let t55517 = t4067 * t4067;
    let t55530 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t55420 - t64 * t656 * (t55457 + t55512) / F::cast_from(8.0_f64) + t64 * t2331 * t55517 / F::cast_from(2.0_f64) + t45421 + F::cast_from(308.0_f64) / F::cast_from(27.0_f64) * t45656 + F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t45658 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t45660 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t45662 + F::cast_from(308.0_f64) / F::cast_from(27.0_f64) * t45422 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t45424 - F::cast_from(11.0_f64) / F::cast_from(9.0_f64) * t45426 - F::cast_from(44.0_f64) / F::cast_from(9.0_f64) * t45688 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t45690;
    t55530
}
