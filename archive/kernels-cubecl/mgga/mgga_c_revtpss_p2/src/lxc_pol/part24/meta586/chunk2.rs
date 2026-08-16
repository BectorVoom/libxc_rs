//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1823/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1823<F: Float>(t1394: F, t1877: F, t1879: F, t22229: F, t22236: F, t225: F, t22809: F, t22936: F, t22944: F, t22947: F, t22950: F, t4049: F, t47171: F, t539: F, t541: F, t5650: F, t5651: F, t6816: F, t6832: F, t6837: F, t6840: F, t91826: F, t91870: F, t91875: F, t91957: F, t91964: F, t91967: F, t91971: F, t91981: F, t92017: F, t92023: F, t92030: F) -> F {
    let t92063 = -(t91957 + t91964 + t91967 + t91971 + t91981 + t92017 + t92023 + t92030) * t225 * t541 + F::cast_from(12.0_f64) * t22936 * t1879 - F::cast_from(72.0_f64) * t6832 * t6837 + F::cast_from(18.0_f64) * t6832 * t6840 + F::cast_from(240.0_f64) * t1877 * t22944 - F::cast_from(144.0_f64) * t22229 * t22947 + F::cast_from(12.0_f64) * t1877 * t22950 - F::cast_from(360.0_f64) * t539 * t47171 * t91870 + F::cast_from(360.0_f64) * t5650 * t22236 * t6816 - F::cast_from(36.0_f64) * t539 * t4049 * t91875 - F::cast_from(48.0_f64) * t5650 * t5651 * t22809 + F::cast_from(3.0_f64) * t539 * t1394 * t91826;
    t92063
}
