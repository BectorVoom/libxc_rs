//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1743;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1744;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta428(t22690: f64, t6968: f64, t22642: f64, t1351: f64, t1372: f64, t550: f64, t6976: f64, t1992: f64, t12272: f64, t268: f64, t534: f64, t6559: f64, t1338: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22691, t22693, t22695, t22696, t22697, t22699, t22700, t22701, t22704) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1743(t22690, t6968, t22642, t1351, t1372, t550, t6976, t1992, t12272, t268, t534, t6559);
        let t22705 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1744(t1338, t22690);
    (t22691, t22693, t22695, t22696, t22697, t22699, t22700, t22701, t22704, t22705)
}
