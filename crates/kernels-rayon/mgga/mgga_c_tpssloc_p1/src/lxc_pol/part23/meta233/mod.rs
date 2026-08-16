//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta233 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk883;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk884;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta233(t118: f64, t5544: f64, t794: f64, t2576: f64, t2563: f64, t5555: f64, t252: f64, t5584: f64, t1499: f64, t4290: f64, t4166: f64, t4177: f64, t120: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t16791, t16792, t16794, t16815, t16830, t16836) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk883(t118, t5544, t794, t2576, t2563, t5555, t252, t5584, t1499, t4290, t4166, t4177);
        let t16839 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk884(t120, t5584);
    (t16791, t16792, t16794, t16815, t16830, t16836, t16839)
}
