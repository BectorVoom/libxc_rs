//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1305/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1305(t33464: f64, t2464: f64, t2465: f64, t825: f64, t8528: f64, t10837: f64, t7416: f64, t24751: f64, t2676: f64, t24945: f64, t11116: f64, t22263: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33465 = 0.19171462976960374838e0_f64 * t33464;
    let t33468 = t825 * t2464 * t2465 * t8528;
    let t33469 = 0.85206502119823888168e-1_f64 * t33468;
    let t33473 = t7416 * t10837;
    let t33474 = 0.51123901271894332902e0_f64 * t33473;
    let t33476 = 0.23833659967900284446e0_f64 * t24751 * t2676;
    let t33478 = 0.23833659967900284446e0_f64 * t24945 * t2676;
    let t33480 = 0.15889106645266856297e0_f64 * t22263 * t11116;
    (t33465, t33469, t33474, t33476, t33478, t33480)
}
