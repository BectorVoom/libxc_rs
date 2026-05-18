//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 976/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk976<F: Float>(t27213: F, t7407: F, t1956: F, t26508: F, t26521: F, t26522: F, t26529: F, t26534: F, t26536: F, t26538: F, t27199: F, t28400: F, t28405: F, t28411: F, t28418: F, t4487: F, t7070: F, t7403: F, t7420: F) -> F {
    let t28422 = t27213 * t7407;
    let t28424 = -F::new(0.4336814094102599731e0) * t1956 * t28400 + F::new(0.4336814094102599731e0) * t7070 * t28405 + F::new(0.4336814094102599731e0) * t27199 * t7420 - F::new(0.26020884564615598386e1) * t7070 * t28411 + t26508 + F::new(0.13170898365871023197e1) * t7403 * t4487 + t26521 - F::new(0.12851425765524037203e-1) * t26522 + F::new(0.8673628188205199462e0) * t7070 * t28418 + F::new(0.72280234901709995518e-2) * t26529 - t26534 - t26536 - t26538 + F::new(0.72280234901709995518e-2) * t28422;
    t28424
}
