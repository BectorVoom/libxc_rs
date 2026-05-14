//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 927/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk927<F: Float>(t30232: F, t30953: F, t31151: F, t31194: F, t504: F, t2282: F, t27047: F, t20922: F, t8189: F, t6241: F, t8286: F, t14294: F, t4170: F, t30947: F, t467: F, t492: F) -> (F, F, F, F, F, F, F) {
    let t31196 = t30232 + t30953 + t31151 + t31194;
    let t31197 = t31196 * t504;
    let t31199 = 3.0 * t27047 * t2282;
    let t31201 = 6.0 * t20922 * t8189;
    let t31203 = 3.0 * t6241 * t8286;
    let t31204 = t8189 * t2282;
    let t31206 = 6.0 * t14294 * t31204;
    let t31207 = t2282 * t8286;
    let t31209 = 6.0 * t4170 * t31207;
    let t31210 = t30947 * t467;
    let t31211 = t31210 * t492;
    (t31197, t31199, t31201, t31203, t31206, t31209, t31211)
}
