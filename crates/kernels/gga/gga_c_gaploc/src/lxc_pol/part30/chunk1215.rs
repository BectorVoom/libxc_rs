//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1215/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1215<F: Float>(t24487: F, t2508: F, t948: F, t2586: F, t8637: F, t29277: F, t7064: F, t8970: F, t3431: F, t701: F, t2610: F, t10752: F, t5288: F) -> (F, F, F, F, F, F) {
    let t32253 = F::new(0.23071578690426672851e-1) * t2508 * t24487 * t948;
    let t32256 = F::new(0.46143157380853345702e-1) * t2508 * t8637 * t2586;
    let t32258 = t7064 * t29277 * t8970;
    let t32259 = F::new(0.1281754371690370714e-2) * t32258;
    let t32260 = t3431 * t701;
    let t32261 = t2610 * t32260;
    let t32266 = F::new(0.46143157380853345702e-1) * t5288 * t10752;
    (t32253, t32256, t32259, t32260, t32261, t32266)
}
