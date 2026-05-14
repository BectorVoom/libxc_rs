//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 713/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk713<F: Float>(t3722: F, t954: F, t2508: F, t13861: F, t2580: F, t12255: F, t948: F, t13180: F, t13187: F, t13189: F, t13193: F, t13197: F, t13198: F, t13204: F, t13215: F, t13216: F, t13220: F) -> (F, F, F, F) {
    let t13918 = t954 * t3722;
    let t13919 = t2508 * t13918;
    let t13921 = t2580 * t13861;
    let t13922 = t2508 * t13921;
    let t13924 = t12255 * t948;
    let t13925 = t2508 * t13924;
    let t13930 = 0.76905262301422242837e-2 * t13919 + 0.15381052460284448567e-1 * t13922 - 0.23071578690426672851e-1 * t13925 + t13216 + t13220 - t13197 + t13198 - t13215 - 0.23071578690426672851e-1 * t13180 - t13187 + 0.76905262301422242837e-2 * t13189 + t13193 + 0.15381052460284448567e-1 * t13204;
    (t13918, t13921, t13924, t13930)
}
