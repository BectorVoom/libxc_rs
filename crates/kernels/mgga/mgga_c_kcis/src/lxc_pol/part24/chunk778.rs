//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 778/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk778<F: Float>(t169: F, t2628: F, t174: F, t2640: F, t1709: F, t9985: F, t2861: F, t5027: F, t5030: F, t1094: F, t4922: F, t1775: F, t9528: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13003 = F::cast_from(1.0_f64) / t2628 / t169;
    let t13014 = F::cast_from(1.0_f64) / t2640 / t174;
    let t13097 = t1709 * t9985;
    let t13101 = t2861 * t5027;
    let t13102 = F::cast_from(0.33163888888888888888e-2_f64) * t13101;
    let t13103 = t2861 * t5030;
    let t13105 = t4922 * t1094;
    let t13106 = t13105 * sigma0;
    let t13122 = t9528 * t1775;
    (t13003, t13014, t13097, t13101, t13102, t13103, t13105, t13106, t13122)
}
