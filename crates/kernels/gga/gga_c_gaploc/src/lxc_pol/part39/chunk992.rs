//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 992/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk992<F: Float>(t204: F, t47803: F, t587: F, t2487: F, t6711: F, t4130: F, t46849: F, t4781: F, t590: F, t41621: F, t41624: F, t41627: F, t41630: F, t41631: F, t41636: F, t41640: F, t41643: F, t41646: F) -> (F,) {
    let t47805 = t587 * t204 * t47803;
    let t47808 = t2487 * t6711 * t47803;
    let t47812 = t4781 * t4130 * t46849 * t590;
    let t47819 = -0.92023022289409799224e1 * t47805 + 0.43710935587469654631e2 * t47808 + 0.15337170381568299871e1 * t47812 + 0.29792074959875355558e-1 * t41621 + t41624 + t41627 + t41630 + 0.19171462976960374838e0 * t41631 + 0.19171462976960374838e0 * t41636 - 0.42603251059911944084e-1 * t41640 - 0.14896037479937677779e-1 * t41643 + t41646;
    (t47819,)
}
