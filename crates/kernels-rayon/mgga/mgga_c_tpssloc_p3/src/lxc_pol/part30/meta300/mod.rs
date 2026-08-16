//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta300 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1318;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta300(t10027: f64, t222: f64, t805: f64, t9541: f64, t2627: f64, t852: f64, t856: f64, t68: f64, t261: f64, t2751: f64, t1053: f64, t1887: f64, t337: f64, t615: f64, t134: f64, t976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10029, t10036, t10054, t10108, t10109, t10110, t10143) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1318(t10027, t222, t805, t9541, t2627, t852, t856, t68, t261, t2751);
        let (t10163, t10164, t10165, t10186, t10189) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1319(t1053, t68, t1887, t337, t615, t134, t976);
    (t10029, t10036, t10054, t10108, t10109, t10110, t10143, t10163, t10164, t10165, t10186, t10189)
}
