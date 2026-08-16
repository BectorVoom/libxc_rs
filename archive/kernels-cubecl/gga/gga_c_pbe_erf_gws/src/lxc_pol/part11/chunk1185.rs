//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1185/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1185<F: Float>(t41339: F, t48067: F, t48069: F, t48071: F, t48076: F, t48078: F, t48080: F, t48082: F, t48084: F, t48086: F, t48088: F, t17548: F, t26328: F, t48090: F, t48092: F, t48095: F, t48099: F, t48101: F, t48102: F, t48103: F, t48104: F, t48105: F) -> (F, F) {
    let t48667 = t48067 + t48069 + t48071 + t48076 + t48078 + t48080 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t41339 - t48082 - t48084 + t48086 + t48088;
    let t48669 = -t48090 + t48092 - t48095 - t48099 - t48101 + F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t26328 - t48102 + t48103 - t48104 - t48105 + t17548;
    (t48667, t48669)
}
