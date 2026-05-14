//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 871/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk871<F: Float>(t425: F, t5926: F, t3521: F, t5923: F, t16940: F, t5915: F, t5929: F, t5900: F, t5904: F, t5908: F, t5896: F, t13220: F, t459: F, t1354: F, t3564: F, t1175: F, t2191: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19244 = t5926 * t425;
    let t19255 = 0.13140859333333333333e-2 * t3521 * t5923;
    let t19271 = t16940 * t5915;
    let t19278 = 0.98556445e-3 * t3521 * t5929;
    let t19318 = 0.13140859333333333334e-2 * t3521 * t5900;
    let t19320 = 0.8760572888888888889e-3 * t3521 * t5904;
    let t19322 = 0.17521145777777777778e-2 * t3521 * t5908;
    let t19324 = 0.14600954814814814815e-2 * t3521 * t5896;
    let t19330 = t13220 * t459;
    let t19351 = t3564 * t1354;
    let t19352 = t2191 * t1175;
    (t19244, t19255, t19271, t19278, t19318, t19320, t19322, t19324, t19330, t19351, t19352)
}
