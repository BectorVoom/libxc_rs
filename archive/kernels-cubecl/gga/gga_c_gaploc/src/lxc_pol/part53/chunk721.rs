//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 721/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk721<F: Float>(t12207: F, t955: F, t3722: F, t954: F, t2508: F, t13861: F, t2580: F, t12255: F, t948: F, t12223: F, t2562: F, t883: F) -> (F, F, F, F, F, F, F, F) {
    let t13906 = t955 * t12207;
    let t13918 = t954 * t3722;
    let t13919 = t2508 * t13918;
    let t13921 = t2580 * t13861;
    let t13922 = t2508 * t13921;
    let t13924 = t12255 * t948;
    let t13925 = t2508 * t13924;
    let t13934 = t2562 * t883 * t12223;
    (t13906, t13918, t13919, t13921, t13922, t13924, t13925, t13934)
}
