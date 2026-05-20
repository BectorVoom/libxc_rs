//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2182/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2182<F: Float>(t106554: F, t27799: F, t18838: F, t33: F, t106482: F, t106516: F, t108002: F, t108005: F, t108009: F, t108021: F, t108028: F, t108030: F, t108033: F, t1711: F, t1940: F, t1963: F, t2403: F, t27158: F, t27364: F, t27368: F, t27382: F, t27810: F, t27817: F, t29964: F, t4541: F, t7091: F, t7207: F, t7783: F, t93404: F) -> F {
    let t108036 = t27799 * t106554;
    let t108043 = t33 * t18838;
    let t108047 = -F::new(3.0) * t27158 * t108002 - t1940 * t7091 * t108005 / F::new(2.0) + F::new(3.0) * t4541 * t1963 * t108009 + F::new(3.0) * t2403 * t7783 * t27810 - t1940 * t106516 * t7207 / F::new(2.0) + t1940 * t27364 * t1711 - t1940 * t7091 * t108021 / F::new(2.0) + t1940 * t106482 * t33 / F::new(2.0) + t27382 * t108028 + F::new(3.0) * t27158 * t108030 + F::new(6.0) * t27158 * t108033 + F::new(2.0) * t27382 * t108036 - t1940 * t27368 * t27817 + t1940 * t93404 * t29964 - t1940 * t7091 * t108043 / F::new(2.0);
    t108047
}
