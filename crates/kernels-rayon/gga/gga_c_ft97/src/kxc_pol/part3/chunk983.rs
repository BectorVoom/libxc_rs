//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 983/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk983(t19216: f64, t19229: f64, t1196: f64, t284: f64, t1111: f64, t1472: f64, t19101: f64, t19103: f64, t19107: f64, t19109: f64, t19117: f64, t19121: f64, t19125: f64, t19129: f64, t19132: f64, t19135: f64, t19144: f64, t19147: f64, t19203: f64, t285: f64, t4062: f64, t4089: f64, t4090: f64, t4094: f64, t4099: f64, t4104: f64, t5231: f64, t817: f64, t821: f64) -> f64 {
    let t19230 = t19216 + t19229;
    let t19233 = t1196 * t284;
    let t19238 = -0.58388215305914623988e1_f64 * t19101 * t19103 + 0.29194107652957311994e1_f64 * t19107 * t19109 + 0.2416365355361531912e1_f64 * t4062 * t1111 + 0.2416365355361531912e1_f64 * t4090 * t1111 - 0.2416365355361531912e1_f64 * t19117 * t1111 - 0.1208182677680765956e1_f64 * t1472 * t19121 - 0.7651823625311517721e1_f64 * t4104 * t19125 - 0.38259118126557588605e1_f64 * t4099 * t19129 + 0.29194107652957311994e1_f64 * t19132 * t19103 - 0.14597053826478655997e1_f64 * t19135 * t19109 + 0.1208182677680765956e1_f64 * t4099 * t19121 + 0.7651823625311517721e1_f64 * t4094 * t19125 + 0.38259118126557588605e1_f64 * t1472 * t19129 - 0.2416365355361531912e1_f64 * t19144 * t1111 + 2.0_f64 * t19147 + 2.0_f64 * t19203 - t285 * t817 * t19230 + 4.0_f64 * t19233 * t4089 - 2.0_f64 * t5231 * t821;
    t19238
}
