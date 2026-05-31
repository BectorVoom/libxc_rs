//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 710/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk710<F: Float>(t5805: F, t5813: F, t1513: F, t5809: F, t1544: F, t156: F, t496: F, t506: F, t5683: F, t102: F, t505: F, t96: F) -> (F, F, F, F, F, F, F) {
    let t5814 = t5813 * t5805;
    let t5815 = F::cast_from(0.2923025e1_f64) * t5814;
    let t5816 = t1513 * t5809;
    let t5817 = F::cast_from(0.19486833333333333333e1_f64) * t5816;
    let t5818 = t156 * t1544;
    let t5819 = t496 * t5818;
    let t5821 = t506 * t5683;
    let t5823 = F::cast_from(0.1753815e2_f64) * t102 * t5821;
    let t5825 = F::cast_from(1.0_f64) / t505 / t96;
    (t5815, t5817, t5818, t5819, t5821, t5823, t5825)
}
