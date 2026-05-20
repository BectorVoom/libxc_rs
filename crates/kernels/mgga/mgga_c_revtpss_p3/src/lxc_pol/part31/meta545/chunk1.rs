//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1934/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1934<F: Float>(t1959: F, t25333: F, t25337: F, t25362: F, t25364: F, t25371: F, t25391: F, t25406: F, t25424: F, t27199: F, t27280: F, t27325: F, t27335: F, t27338: F, t27342: F, t27344: F, t29675: F, t29683: F, t29691: F, t29695: F, t29698: F, t7070: F, t7775: F) -> F {
    let t29703 = F::cast_from(0.4336814094102599731e0_f64) * t7070 * t29675 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t7775 + t25333 - F::cast_from(0.25702851531048074406e-1_f64) * t27280 - t25337 - t25362 - t25364 + t25371 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t29683 - F::cast_from(0.19514881078765566038e-1_f64) * t27325 - t25406 + F::cast_from(0.10975748638225852664e-1_f64) * t27335 + F::cast_from(0.14456046980341999104e-1_f64) * t27338 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t29691 - F::cast_from(0.8673628188205199462e0_f64) * t7070 * t29695 + t25424 - F::cast_from(0.4336814094102599731e0_f64) * t29698 * t1959 - F::cast_from(0.28912093960683998208e-1_f64) * t27342 + F::cast_from(0.51405703062096148812e-1_f64) * t27344;
    t29703
}
