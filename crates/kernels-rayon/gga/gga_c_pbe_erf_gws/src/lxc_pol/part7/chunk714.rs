//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 714/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk714(t133: f64, t5783: f64, t5818: f64, t1533: f64, t481: f64, t5787: f64, t2911: f64, t2912: f64, t5753: f64, t5755: f64, t5771: f64, t5776: f64, t5779: f64, t5791: f64, t5797: f64, t5799: f64, t5815: f64, t5817: f64, t5823: f64, t5831: f64, t5863: f64) -> (f64, f64) {
    let t5864 = t133 * t5783;
    let t5866 = t133 * t5818;
    let t5870 = t481 * t1533;
    let t5874 = t133 * t5787;
    let t5878 = -t5863 - 0.22990066666666666666e1_f64 * t5864 - t5823 + t5831 + t5753 + t5771 - t5755 + t5797 - t5779 - 0.51727649999999999999e1_f64 * t5866 - 0.2069106e2_f64 * t133 * t5799 + 0.15518295e2_f64 * t2911 * t2912 * t5870 - t5815 - t5817 + 0.1724255e1_f64 * t5874 - 0.1724255e1_f64 * t133 * t5791 - t5776;
    (t5870, t5878)
}
