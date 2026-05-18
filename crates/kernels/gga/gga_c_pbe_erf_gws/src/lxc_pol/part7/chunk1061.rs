//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1061/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1061<F: Float>(t101: F, t1501: F, t159: F, t16580: F, t19087: F, t19107: F, t19108: F, t19117: F, t19121: F, t19124: F, t19129: F, t19132: F, t19136: F, t19138: F, t19140: F, t19143: F, t19148: F, t19152: F, t19157: F, t19161: F, t19165: F, t19169: F, t2033: F, t2035: F, t2037: F, t281: F, t285: F, t523: F, t5603: F, t5881: F, t8331: F) -> F {
    let t19170 = t19107 + F::new(12.0) * t2035 * t19108 - F::new(0.11974234010254609094e-1) * t281 * t16580 * t159 * t285 - F::new(0.47896936041018436376e-1) * t19117 - t19121 - F::new(3.0) * t5881 * t2033 + F::new(12.0) * t19124 * t2037 + F::new(6.0) * t101 * t19087 * t19129 + F::new(36.0) * t19132 * t5603 + t523 * t19136 - F::new(36.0) * t19138 * t19140 + F::new(72.0) * t8331 * t19143 - F::new(0.71845404061527654564e-1) * t19148 - F::new(0.47896936041018436376e-1) * t19152 - t19157 + F::new(3.0) * t5881 * t1501 + F::new(6.0) * t523 * t19161 - F::new(0.26861343269868796571e-1) * t19165 - t19169;
    t19170
}
