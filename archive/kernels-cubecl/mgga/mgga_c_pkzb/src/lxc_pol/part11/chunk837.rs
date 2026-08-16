//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 837/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk837<F: Float>(t3406: F, t568: F, t581: F, t1706: F, t2592: F, t5225: F, t5265: F, t6873: F, t6885: F, t6894: F, t6914: F, t6928: F, t6933: F, t8921: F, t8924: F, t8926: F, t8931: F, t8935: F) -> (F, F) {
    let t8939 = t581 * t3406 * t568;
    let t8944 = F::cast_from(0.85748036236139473944e-3_f64) * t2592 * t8921 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t8924 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8926 - F::cast_from(0.80031500487063509016e-2_f64) * t6873 - F::cast_from(0.80031500487063509015e-2_f64) * t6885 - t6894 - t5225 * t8931 / F::cast_from(4.0_f64) + t1706 * t8935 / F::cast_from(8.0_f64) + t1706 * t8939 / F::cast_from(16.0_f64) - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t5265 - t6914 + t6928 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t6933;
    (t8939, t8944)
}
