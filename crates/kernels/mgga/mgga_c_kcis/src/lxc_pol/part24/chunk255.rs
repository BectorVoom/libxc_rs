//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 255/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk255<F: Float>(t1079: F, t922: F, t1030: F, t104: F, t1050: F, t1055: F, t1057: F, t1063: F, t1065: F, t1069: F, t1072: F, t1078: F, t111: F, t120: F, t829: F) -> (F, F) {
    let t1080 = t1079 * t922;
    let t1083 = t1050 + F::cast_from(0.11955719325063177623e-1_f64) * t1030 * t829 - t1055 - F::new(0.3513e-2) * t104 * t1057 + t1063 + F::new(0.7925e-3) * t111 * t1065 - t1069 - F::cast_from(0.5179538907796306876e-4_f64) * t1072 * t829 + t1078 + F::new(0.50413125e-5) * t120 * t1080;
    (t1080, t1083)
}
