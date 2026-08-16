//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2413/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2413<F: Float>(t21238: F, t2929: F, t4497: F, t959: F, t17934: F, t4489: F, t4498: F, t17565: F, t21089: F, t41825: F, t17951: F, t4483: F) -> (F, F, F, F, F) {
    let t68902 = t2929 * t21238;
    let t68905 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t68902 * t4497;
    let t68910 = F::cast_from(0.35089341735807877242e1_f64) * t17934 * t4489;
    let t68912 = F::cast_from(0.51947577317044391276e2_f64) * t17934 * t4498;
    let t68916 = F::cast_from(0.12304822629859687989e5_f64) * t959 * t41825 * t21089 * t17565;
    let t68918 = F::cast_from(0.70178683471615754484e1_f64) * t4483 * t17951;
    (t68905, t68910, t68912, t68916, t68918)
}
