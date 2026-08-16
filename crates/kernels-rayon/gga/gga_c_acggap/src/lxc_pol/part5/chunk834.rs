//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 834/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk834(t2795: f64, t687: f64, t2792: f64, t286: f64, t680: f64, t2617: f64, t2620: f64, t195: f64, t2987: f64, t656: f64, t4: f64, t657: f64, t901: f64) -> (f64, f64, f64, f64, f64) {
    let t11649 = t2795 * t687;
    let t11652 = 0.61524113149298439947e4_f64 * t286 * t2792 * t680 * t11649;
    let t11653 = t2617 * t2620;
    let t11657 = 0.1301229756036208781e0_f64 * t656 * t195 * t2987;
    let t11659 = t901 * t4 * t657;
    (t11649, t11652, t11653, t11657, t11659)
}
