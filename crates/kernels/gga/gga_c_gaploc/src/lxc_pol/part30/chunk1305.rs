//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1305/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1305<F: Float>(t33464: F, t2464: F, t2465: F, t825: F, t8528: F, t10837: F, t7416: F, t24751: F, t2676: F, t24945: F, t11116: F, t22263: F) -> (F, F, F, F, F, F) {
    let t33465 = F::cast_from(0.19171462976960374838e0_f64) * t33464;
    let t33468 = t825 * t2464 * t2465 * t8528;
    let t33469 = F::cast_from(0.85206502119823888168e-1_f64) * t33468;
    let t33473 = t7416 * t10837;
    let t33474 = F::cast_from(0.51123901271894332902e0_f64) * t33473;
    let t33476 = F::cast_from(0.23833659967900284446e0_f64) * t24751 * t2676;
    let t33478 = F::cast_from(0.23833659967900284446e0_f64) * t24945 * t2676;
    let t33480 = F::cast_from(0.15889106645266856297e0_f64) * t22263 * t11116;
    (t33465, t33469, t33474, t33476, t33478, t33480)
}
