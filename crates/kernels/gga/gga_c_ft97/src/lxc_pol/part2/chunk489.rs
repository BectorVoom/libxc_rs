//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 489/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk489<F: Float>(t3157: F, t457: F, t91: F, t2981: F, t3006: F, t1549: F, t1552: F, t1832: F, t2986: F, t2990: F, t2995: F, t3003: F, t3011: F, t3016: F, t3106: F, t3121: F) -> (F, F) {
    let t3159 = t91 * t457 * t3157;
    let t3161 = t2981 / 27.0;
    let t3166 = t3006 / 9.0;
    let t3170 = -t3121 / 12.0 + t3159 / 6.0 + t1832 + t1549 + t1552 + t3161 - 2.0 / 27.0 * t2986 + t2990 / 9.0 + 2.0 / 9.0 * t2995 - 2.0 / 9.0 * t3003 + t3166 + t3011 / 9.0 + 2.0 / 3.0 * t3016 - t3106 / 3.0;
    (t3159, t3170)
}
