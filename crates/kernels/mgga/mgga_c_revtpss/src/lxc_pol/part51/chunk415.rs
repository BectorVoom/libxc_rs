//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 415/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk415<F: Float>(t2022: F, t545: F, t2028: F, t2024: F, t2027: F, t213: F) -> (F, F, F) {
    let t2029 = t545 * t2022;
    let t2030 = t2028 * t2029;
    let t2033 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t2024 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2030;
    (t2029, t2030, t2033)
}
