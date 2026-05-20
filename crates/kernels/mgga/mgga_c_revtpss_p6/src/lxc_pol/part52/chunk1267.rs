//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1267/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1267<F: Float>(t125886: F, t121076: F, t121327: F, t121337: F, t122346: F, t122498: F, t122503: F, t122504: F, t125873: F, t125875: F, t125901: F, t125903: F, t27972: F) -> F {
    let t128833 = F::cast_from(0.3718732920905101082e-4_f64) * t125886;
    let t128837 = F::cast_from(0.7437465841810202164e-3_f64) * t125873 + F::cast_from(0.14874931683620404328e-2_f64) * t125875 + t122498 + F::cast_from(0.3427184259906141157e1_f64) * t121076 * t122346 * t27972 + t128833 - F::cast_from(0.66934509195437693771e-4_f64) * t121327 + t121337 + F::cast_from(0.37645955677973955999e-4_f64) * t125901 - F::cast_from(0.66934509195437693771e-4_f64) * t125903 - t122503 + t122504;
    t128837
}
