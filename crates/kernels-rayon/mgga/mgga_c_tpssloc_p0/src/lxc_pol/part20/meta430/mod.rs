//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta430(t225: f64, t4553: f64, t1634: f64, t3206: f64, t3174: f64, t4559: f64, t4555: f64, t4657: f64, t990: f64, t14488: f64, t381: f64, t1060: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14545, t14549, t14552, t14555, t14562, t14571, t14572) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1850(t225, t4553, t1634, t3206, t3174, t4559, t4555, t4657, t990, t14488, t381, t1060);
    (t14545, t14549, t14552, t14555, t14562, t14571, t14572)
}
