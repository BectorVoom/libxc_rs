//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1976/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1976<F: Float>(t102293: F, t102296: F, t102298: F, t102306: F, t102309: F, t102316: F, t108178: F, t25930: F, t26304: F, t28008: F, t28915: F, t8104: F, t96280: F, t96284: F, t96287: F, t96289: F, t96298: F, t97933: F) -> F {
    let t109533 = -F::cast_from(0.34270468708064099208e-2_f64) * t96280 - F::cast_from(0.8673628188205199462e0_f64) * t28008 * t8104 - F::cast_from(0.68540937416128198416e-1_f64) * t102293 - F::cast_from(0.19274729307122665472e-1_f64) * t102296 - F::cast_from(0.17347256376410398924e1_f64) * t25930 * t26304 * t108178 - t96284 + F::cast_from(0.34270468708064099208e-1_f64) * t102298 + t102306 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t28915 - F::cast_from(0.22849835011101738147e-2_f64) * t96287 + t102309 + F::cast_from(0.17135234354032049604e-1_f64) * t96289 + F::cast_from(0.19274729307122665472e-1_f64) * t102316 + F::cast_from(0.96373646535613327357e-2_f64) * t96298;
    t109533
}
