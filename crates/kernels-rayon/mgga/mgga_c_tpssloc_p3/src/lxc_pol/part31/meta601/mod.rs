//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1846;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta601(t87247: f64, t87255: f64, t87262: f64, t87270: f64, t87272: f64, t87291: f64, t87293: f64, t87300: f64, t87308: f64, t87328: f64, t87330: f64, t87332: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92599, t92603, t92607, t92614, t92615, t92626, t92627, t92630, t92635, t92645, t92646, t92647) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1846(t87247, t87255, t87262, t87270, t87272, t87291, t87293, t87300, t87308, t87328, t87330, t87332);
    (t92599, t92603, t92607, t92614, t92615, t92626, t92627, t92630, t92635, t92645, t92646, t92647)
}
