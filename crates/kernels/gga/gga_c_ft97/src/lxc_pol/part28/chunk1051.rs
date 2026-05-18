//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1051/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1051<F: Float>(t136825: F, t34476: F, t32279: F, t136847: F, t136520: F, t136679: F, t136714: F, t145353: F, t22623: F, t3019: F, t3021: F, t3058: F, t3061: F, t3067: F, t3100: F, t32251: F, t32252: F, t32301: F, t58607: F, t66: F, t7171: F, t7172: F, t930: F) -> (F, F, F) {
    let t145468 = t136825 * t34476;
    let t145469 = t32279 * t145468;
    let t145471 = t136847 * t34476;
    let t145474 = -F::new(0.55136259934963963184e-3) * t32251 * t32252 * t58607 * t930 + F::new(0.22227677429409423704e-2) * t22623 * t145353 + F::new(0.26043295784446077722e-6) * t136679 - F::new(0.60548059007656442388e-3) * t136714 + F::new(0.23254900946437792e-1) * t136520 * t3067 + F::new(0.38731446812548799881e-3) * t32301 * t3061 - F::new(0.23254900946437792e-1) * t32301 * t3058 + F::new(0.13519760450715832853e-3) * t3019 * t7171 * t66 * t3021 - F::new(2.0) * t7172 * t3100 + F::new(0.26086440517961693841e-2) * t145469 - F::new(0.20869152414369355073e-1) * t32279 * t145471;
    (t145468, t145471, t145474)
}
