//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1201/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1201<F: Float>(t1398: F, t2022: F, t543: F, t7301: F, t545: F, t7274: F, t2028: F, t1445: F, t2027: F, t2030: F, t213: F, t561: F, t7245: F, t7248: F, t7275: F, t7279: F, t7288: F, t7291: F, t7292: F, t7295: F, t7298: F) -> (F, F, F, F, F) {
    let t7303 = t2022 * t1398 * t543;
    let t7304 = t7301 * t7303;
    let t7307 = t545 * t7274;
    let t7308 = t2028 * t7307;
    let t7311 = -t7245 + t7248 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t7275 * t561 - F::cast_from(0.65854491829355115987e0_f64) * t7279 * t1445 + t7288 - t7291 - F::cast_from(0.4336814094102599731e0_f64) * t7292 * t2030 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7298 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7304 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t7308;
    (t7303, t7304, t7307, t7308, t7311)
}
