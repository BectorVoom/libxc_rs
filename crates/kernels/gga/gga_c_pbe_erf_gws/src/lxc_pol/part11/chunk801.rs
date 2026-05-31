//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 801/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk801<F: Float>(t12898: F, t506: F, t102: F, t10: F, t127: F, t12931: F, t12934: F, t12937: F, t12946: F, t12947: F, t12951: F, t12952: F, t12955: F, t2893: F, t3637: F, t496: F, t5836: F, t8149: F, t8160: F, t8200: F) -> (F, F, F) {
    let t12958 = t506 * t12898;
    let t12960 = F::cast_from(0.1753815e2_f64) * t102 * t12958;
    let t12961 = -t496 * t12931 / F::cast_from(2.0_f64) + t12934 - F::cast_from(0.293808e1_f64) * t8149 - F::cast_from(0.146904e1_f64) * t8160 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t496 * t10 * t12937 + F::cast_from(0.1762848e2_f64) * t127 * t2893 * t3637 + t12946 - t12947 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t8200 + t5836 - t12951 - F::cast_from(0.146904e1_f64) * t127 * t12952 - F::cast_from(0.293808e2_f64) * t127 * t12955 - t12960;
    (t12958, t12960, t12961)
}
