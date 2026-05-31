//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 882/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk882<F: Float>(t16820: F, t218: F, t5108: F, t213: F, t1793: F, t186: F, t211: F, t16781: F, t16787: F, t16792: F, t16796: F, t16800: F, t16806: F, t16811: F, t16814: F, t16818: F) -> (F, F, F) {
    let t16821 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t16820;
    let t16823 = F::cast_from(1.0_f64) / t5108 / t218;
    let t16824 = t213 * t16823;
    let t16825 = t1793 * t1793;
    let t16829 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t211 * t186 * t16824 * t16825;
    let t16830 = -t16781 - t16787 + t16792 + t16796 + t16800 - t16806 - t16811 + t16814 - t16818 + t16821 + t16829;
    (t16821, t16829, t16830)
}
