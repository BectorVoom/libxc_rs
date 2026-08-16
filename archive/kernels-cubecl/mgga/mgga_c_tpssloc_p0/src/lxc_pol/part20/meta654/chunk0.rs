//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2419/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2419<F: Float>(t10750: F, t13723: F, t959: F, t10757: F, t1580: F, t41825: F, t10853: F, t4483: F, t13508: F, t2940: F, t10713: F, t10756: F, t300: F) -> (F, F, F, F, F, F) {
    let t49502 = F::cast_from(0.14035736694323150897e2_f64) * t959 * t13723 * t10750;
    let t49506 = F::cast_from(0.12304822629859687989e5_f64) * t959 * t41825 * t1580 * t10757;
    let t49508 = F::cast_from(0.51947577317044391277e2_f64) * t4483 * t10853;
    let t49510 = F::cast_from(0.51947577317044391277e2_f64) * t2940 * t13508;
    let t49512 = F::cast_from(0.35089341735807877242e1_f64) * t4483 * t10713;
    let t49513 = t300 * t10756;
    (t49502, t49506, t49508, t49510, t49512, t49513)
}
