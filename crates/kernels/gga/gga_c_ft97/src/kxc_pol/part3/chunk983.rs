//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 983/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk983<F: Float>(t19216: F, t19229: F, t1196: F, t284: F, t1111: F, t1472: F, t19101: F, t19103: F, t19107: F, t19109: F, t19117: F, t19121: F, t19125: F, t19129: F, t19132: F, t19135: F, t19144: F, t19147: F, t19203: F, t285: F, t4062: F, t4089: F, t4090: F, t4094: F, t4099: F, t4104: F, t5231: F, t817: F, t821: F) -> F {
    let t19230 = t19216 + t19229;
    let t19233 = t1196 * t284;
    let t19238 = -F::cast_from(0.58388215305914623988e1_f64) * t19101 * t19103 + F::cast_from(0.29194107652957311994e1_f64) * t19107 * t19109 + F::cast_from(0.2416365355361531912e1_f64) * t4062 * t1111 + F::cast_from(0.2416365355361531912e1_f64) * t4090 * t1111 - F::cast_from(0.2416365355361531912e1_f64) * t19117 * t1111 - F::cast_from(0.1208182677680765956e1_f64) * t1472 * t19121 - F::cast_from(0.7651823625311517721e1_f64) * t4104 * t19125 - F::cast_from(0.38259118126557588605e1_f64) * t4099 * t19129 + F::cast_from(0.29194107652957311994e1_f64) * t19132 * t19103 - F::cast_from(0.14597053826478655997e1_f64) * t19135 * t19109 + F::cast_from(0.1208182677680765956e1_f64) * t4099 * t19121 + F::cast_from(0.7651823625311517721e1_f64) * t4094 * t19125 + F::cast_from(0.38259118126557588605e1_f64) * t1472 * t19129 - F::cast_from(0.2416365355361531912e1_f64) * t19144 * t1111 + F::cast_from(2.0_f64) * t19147 + F::cast_from(2.0_f64) * t19203 - t285 * t817 * t19230 + F::cast_from(4.0_f64) * t19233 * t4089 - F::cast_from(2.0_f64) * t5231 * t821;
    t19238
}
