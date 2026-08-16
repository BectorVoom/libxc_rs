//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 712/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk712(t127: f64, t1533: f64, t496: f64, t5788: f64, t5791: f64, t5797: f64, t5799: f64, t5806: f64, t5810: f64, t5815: f64, t5817: f64, t5819: f64, t5823: f64, t5826: f64, t5831: f64, t5836: f64, t5837: f64) -> f64 {
    let t5841 = t5788 / 2.0_f64 - t496 * t5791 / 2.0_f64 + t5797 - 6.0_f64 * t496 * t5799 - 0.881424e1_f64 * t5806 - 0.293808e1_f64 * t5810 - t5815 - t5817 - 3.0_f64 / 2.0_f64 * t5819 - t5823 - 0.293808e2_f64 * t127 * t5826 + t5831 + t5836 + 0.1762848e2_f64 * t127 * t5837 * t1533;
    t5841
}
