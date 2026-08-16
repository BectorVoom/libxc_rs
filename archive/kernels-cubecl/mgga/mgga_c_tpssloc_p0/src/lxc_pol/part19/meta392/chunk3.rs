//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1484/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1484<F: Float>(t11913: F, t44690: F, t11881: F, t11883: F, t11884: F, t11897: F, t11916: F, t1244: F, t1246: F, t3604: F, t3610: F, t3612: F, t3613: F, t3626: F, t44669: F, t44673: F, t44700: F, t44710: F, t44785: F, t44786: F, t44798: F, t45314: F, t45320: F, t45323: F, t45326: F, t470: F, t491: F, t493: F) -> F {
    let t45329 = t44690 * t11913;
    let t45332 = t1244 * t1246 * t44798 * t491 + F::cast_from(24.0_f64) * t11881 * t11883 * t44673 + F::cast_from(6.0_f64) * t3610 * t3612 * t44669 + F::cast_from(12.0_f64) * t3610 * t3612 * t44710 - t44700 * t44785 * t44786 + t45314 * t470 * t493 + F::cast_from(24.0_f64) * t11884 * t45326 + F::cast_from(12.0_f64) * t11897 * t3604 + F::cast_from(4.0_f64) * t11916 * t45329 + F::cast_from(12.0_f64) * t3613 * t45320 - F::cast_from(6.0_f64) * t3626 * t45323;
    t45332
}
