//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 263/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk263<F: Float>(t286: F, t912: F, t659: F, t706: F, t711: F, t714: F, t717: F, t753: F, t757: F, t774: F, t782: F, t809: F, t910: F) -> (F, F) {
    let t913 = t286 * t912;
    let t914 = F::cast_from(0.11696447245269292414e1_f64) * t913;
    let t915 = t711 + t714 - t717 - t753 + t910 + t774 + t782 + t659 + t809 + t914 - t706 - t757;
    (t914, t915)
}
