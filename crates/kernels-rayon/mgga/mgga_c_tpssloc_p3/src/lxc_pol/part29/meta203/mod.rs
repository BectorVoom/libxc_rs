//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta203(t381: f64, t4552: f64, t1049: f64, t1603: f64, t1604: f64, t225: f64, t1625: f64, t990: f64, t4343: f64, t977: f64, t2979: f64, t4338: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t4553, t4555, t4557, t4559, t4562, t4565) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1020(t381, t4552, t1049, t1603, t1604, t225, t1625, t990, t4343, t977, t2979, t4338);
    (t4553, t4555, t4557, t4559, t4562, t4565)
}
