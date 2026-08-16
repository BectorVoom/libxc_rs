//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2196;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta610(t3540: f64, t3567: f64, t11159: f64, t11539: f64, t1174: f64, t374: f64, t485: f64, t486: f64, t9697: f64, t1090: f64, t3493: f64, t11786: f64, t3490: f64, t11154: f64, t11784: f64, t1227: f64, t248: f64, t11814: f64, t3572: f64, t11825: f64, t3523: f64, t11820: f64, t3536: f64, t11778: f64, t121: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t45224, t45227, t45250, t45251, t45256) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2196(t3540, t3567, t11159, t11539, t1174, t374, t485, t486, t9697, t1090, t3493, t11786, t3490);
        let (t45260, t45262, t45264, t45266, t45268) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2197(t11154, t11784, t1227, t248, t11814, t3572, t11825, t3523, t11820, t3536, t11778, t121);
    (t45224, t45227, t45250, t45251, t45256, t45260, t45262, t45264, t45266, t45268)
}
