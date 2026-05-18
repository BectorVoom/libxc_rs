//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 981/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk981<F: Float>(t10677: F, t550: F, t1843: F, t10627: F, t688: F, t779: F, t2508: F, t296: F, t3431: F, t123: F, t734: F, t2554: F, t2932: F) -> (F, F, F, F, F, F, F, F) {
    let t10678 = t550 * t10677;
    let t10679 = t1843 * t10678;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = F::new(0.76905262301422242837e-2) * t2508 * t10683;
    let t10686 = t296 * t3431;
    let t10687 = t10686 * t123;
    let t10688 = t10687 * t734;
    let t10691 = t2932 * t2554;
    (t10678, t10679, t10682, t10683, t10685, t10686, t10688, t10691)
}
