//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 714/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk714<F: Float>(t133: F, t5783: F, t5818: F, t1533: F, t481: F, t5787: F, t2911: F, t2912: F, t5753: F, t5755: F, t5771: F, t5776: F, t5779: F, t5791: F, t5797: F, t5799: F, t5815: F, t5817: F, t5823: F, t5831: F, t5863: F) -> (F, F) {
    let t5864 = t133 * t5783;
    let t5866 = t133 * t5818;
    let t5870 = t481 * t1533;
    let t5874 = t133 * t5787;
    let t5878 = -t5863 - F::cast_from(0.22990066666666666666e1_f64) * t5864 - t5823 + t5831 + t5753 + t5771 - t5755 + t5797 - t5779 - F::cast_from(0.51727649999999999999e1_f64) * t5866 - F::new(0.2069106e2) * t133 * t5799 + F::new(0.15518295e2) * t2911 * t2912 * t5870 - t5815 - t5817 + F::new(0.1724255e1) * t5874 - F::new(0.1724255e1) * t133 * t5791 - t5776;
    (t5870, t5878)
}
