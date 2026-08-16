//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 974/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk974<F: Float>(t508: F, t8406: F, t569: F, t1911: F, t2198: F, t1312: F, t2199: F, t2201: F, t4248: F, t651: F, t7732: F, t7889: F, t8393: F) -> (F, F, F, F) {
    let t8407 = t508 * t8406;
    let t8411 = t8406 * t569;
    let t8413 = t2198 * t1911;
    let t8416 = F::cast_from(2.0_f64) * t1312 * t8411 + F::cast_from(2.0_f64) * t1312 * t8413 - F::cast_from(2.0_f64) * t2199 * t4248 - F::cast_from(2.0_f64) * t2199 * t7732 + F::cast_from(2.0_f64) * t2201 * t4248 + F::cast_from(2.0_f64) * t2201 * t7889 - F::cast_from(2.0_f64) * t651 * t8393 - F::cast_from(2.0_f64) * t651 * t8407;
    (t8407, t8411, t8413, t8416)
}
