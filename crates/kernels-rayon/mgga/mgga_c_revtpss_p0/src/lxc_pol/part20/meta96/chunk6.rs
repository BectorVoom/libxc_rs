//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 557/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk557(t2479: f64, t2488: f64, t2648: f64, t2653: f64, t2656: f64, t2666: f64, t2672: f64, t2678: f64, t2686: f64, t2691: f64, t2759: f64, t825: f64, t851: f64) -> f64 {
    let t2760 = 0.42874018118069736972e-2_f64 * t851 * t2479 - 0.25410001404642664112e-4_f64 * t2488 - 0.21437009059034868486e-3_f64 * t825 * t2648 + 0.80031500487063509015e-2_f64 * t2653 - 0.85748036236139473944e-3_f64 * t851 * t2656 + 0.14291339372689912324e-4_f64 * t2666 - t2672 - 0.10164000561857065645e-3_f64 * t2678 + t2686 + t2691 + t2759;
    t2760
}
