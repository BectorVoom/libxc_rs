//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 862/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk862<F: Float>(t136898: F, t5824: F, t136891: F, t5821: F, t136992: F, t7335: F, t136986: F, t136457: F, t32806: F, t138873: F, t542: F, t137007: F, t8811: F, t135: F, t7189: F, t136678: F, t23745: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t138891 = t5824 * t136898;
    let t138894 = 0.20139801475612389137e-1 * t5821 * t136891;
    let t138899 = t5821 * t136898;
    let t138924 = t7335 * t136992;
    let t138927 = 0.8891911659407557944e-2 * t7335 * t136986;
    let t138930 = t32806 * t136457;
    let t138961 = t542 * t138873;
    let t138968 = t8811 * t137007;
    let t138969 = t7189 * t135;
    let t138991 = t23745 * t136678;
    (t138891, t138894, t138899, t138924, t138927, t138930, t138961, t138968, t138969, t138991)
}
