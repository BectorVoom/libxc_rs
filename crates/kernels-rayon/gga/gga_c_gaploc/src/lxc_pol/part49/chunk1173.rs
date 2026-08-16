//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1173/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1173(t204: f64, t47803: f64, t587: f64, t2487: f64, t6711: f64, t4130: f64, t46849: f64, t4781: f64, t590: f64, t41621: f64, t41624: f64, t41627: f64, t41630: f64, t41631: f64, t41636: f64, t41640: f64, t41643: f64, t41646: f64) -> f64 {
    let t47805 = t587 * t204 * t47803;
    let t47808 = t2487 * t6711 * t47803;
    let t47812 = t4781 * t4130 * t46849 * t590;
    let t47819 = -0.92023022289409799224e1_f64 * t47805 + 0.43710935587469654631e2_f64 * t47808 + 0.15337170381568299871e1_f64 * t47812 + 0.29792074959875355558e-1_f64 * t41621 + t41624 + t41627 + t41630 + 0.19171462976960374838e0_f64 * t41631 + 0.19171462976960374838e0_f64 * t41636 - 0.42603251059911944084e-1_f64 * t41640 - 0.14896037479937677779e-1_f64 * t41643 + t41646;
    t47819
}
