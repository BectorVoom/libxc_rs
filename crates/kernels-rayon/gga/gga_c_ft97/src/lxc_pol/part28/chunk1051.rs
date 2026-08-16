//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1051/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1051(t136825: f64, t34476: f64, t32279: f64, t136847: f64, t136520: f64, t136679: f64, t136714: f64, t145353: f64, t22623: f64, t3019: f64, t3021: f64, t3058: f64, t3061: f64, t3067: f64, t3100: f64, t32251: f64, t32252: f64, t32301: f64, t58607: f64, t66: f64, t7171: f64, t7172: f64, t930: f64) -> (f64, f64, f64) {
    let t145468 = t136825 * t34476;
    let t145469 = t32279 * t145468;
    let t145471 = t136847 * t34476;
    let t145474 = -0.55136259934963963184e-3_f64 * t32251 * t32252 * t58607 * t930 + 0.22227677429409423704e-2_f64 * t22623 * t145353 + 0.26043295784446077722e-6_f64 * t136679 - 0.60548059007656442388e-3_f64 * t136714 + 0.23254900946437792e-1_f64 * t136520 * t3067 + 0.38731446812548799881e-3_f64 * t32301 * t3061 - 0.23254900946437792e-1_f64 * t32301 * t3058 + 0.13519760450715832853e-3_f64 * t3019 * t7171 * t66 * t3021 - 2.0_f64 * t7172 * t3100 + 0.26086440517961693841e-2_f64 * t145469 - 0.20869152414369355073e-1_f64 * t32279 * t145471;
    (t145468, t145471, t145474)
}
