//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1781/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1781<F: Float>(t27213: F, t7407: F, t1956: F, t26508: F, t26521: F, t26522: F, t26529: F, t26534: F, t26536: F, t26538: F, t27199: F, t28400: F, t28405: F, t28411: F, t28418: F, t4487: F, t7070: F, t7403: F, t7420: F) -> F {
    let t28422 = t27213 * t7407;
    let t28424 = -F::cast_from(0.4336814094102599731e0_f64) * t1956 * t28400 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t28405 + F::cast_from(0.4336814094102599731e0_f64) * t27199 * t7420 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t28411 + t26508 + F::cast_from(0.13170898365871023197e1_f64) * t7403 * t4487 + t26521 - F::cast_from(0.12851425765524037203e-1_f64) * t26522 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t28418 + F::cast_from(0.72280234901709995518e-2_f64) * t26529 - t26534 - t26536 - t26538 + F::cast_from(0.72280234901709995518e-2_f64) * t28422;
    t28424
}
