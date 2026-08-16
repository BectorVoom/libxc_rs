//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2091;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2092;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta661(t24645: f64, t7999: f64, t2121: f64, t3427: f64, t8010: f64, t24574: f64, t27416: f64, t27794: f64, t27441: f64, t85639: f64, t27446: f64, t1751: f64, t225: f64, t461: f64, t27812: f64, t8006: f64, t85660: f64, t23383: f64, t8020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94427, t94436, t94439, t94446, t94451, t94456, t94458) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2091(t24645, t7999, t2121, t3427, t8010, t24574, t27416, t27794, t27441, t85639, t27446, t1751, t225, t461);
        let (t94475, t94476, t94490) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2092(t24574, t27812, t8006, t85660, t23383, t8020);
    (t94427, t94436, t94439, t94446, t94451, t94456, t94458, t94475, t94476, t94490)
}
