//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk997;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta253(t11569: f64, t11571: f64, t3448: f64, t3469: f64, t3451: f64, t2250: f64, t3450: f64, t3449: f64, t3247: f64, t460: f64, t2244: f64, t1176: f64, t134: f64, t1184: f64, t3447: f64, t3475: f64, t11549: f64, t11556: f64, t11558: f64, t11561: f64, t11563: f64, t11566: f64, t1174: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11572, t11575, t11576, t11579, t11580, t11583, t11584, t11585, t11588) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk997(t11569, t11571, t3448, t3469, t3451, t2250, t3450, t3449, t3247, t460, t2244, t1176, t134);
        let (t11589, t11593, t11597) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk998(t11588, t1184, t3451, t3447, t3448, t3475, t11549, t11556, t11558, t11561, t11563, t11566, t11572, t11576, t11580, t11585, t1174);
    (t11575, t11579, t11583, t11584, t11588, t11589, t11593, t11597)
}
