//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 678/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk678<F: Float>(t505: F, t96: F, t5683: F, t102: F, t1533: F, t497: F, t1235: F, t125: F, t128: F, t2: F, t39: F, t1563: F, t481: F, t127: F, t496: F, t5788: F, t5791: F, t5797: F, t5799: F, t5806: F, t5810: F, t5815: F, t5817: F, t5819: F, t5823: F) -> (F, F, F, F, F, F, F) {
    let t5825 = 1.0 / t505 / t96;
    let t5826 = t5825 * t5683;
    let t5831 = 0.1753815e2 * t102 * t497 * t1533;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = 0.32645333333333333334e0 * t5832 * t5833 * t39;
    let t5837 = t1563 * t481;
    let t5841 = t5788 / 2.0 - t496 * t5791 / 2.0 + t5797 - 6.0 * t496 * t5799 - 0.881424e1 * t5806 - 0.293808e1 * t5810 - t5815 - t5817 - 3.0 / 2.0 * t5819 - t5823 - 0.293808e2 * t127 * t5826 + t5831 + t5836 + 0.1762848e2 * t127 * t5837 * t1533;
    (t5825, t5826, t5831, t5832, t5833, t5837, t5841)
}
