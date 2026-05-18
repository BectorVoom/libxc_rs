//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 771/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk771<F: Float>(t32474: F, t83: F, t452: F, t499: F, t7211: F, t110: F, t32325: F, t1882: F, t7276: F, t1851: F, t7274: F, t379: F) -> (F, F, F, F, F, F) {
    let t32475 = t83 * t32474;
    let t32479 = t452 * t499 * t7211;
    let t32483 = t452 * t110 * t32325;
    let t32487 = F::new(2.0) / F::new(9.0) * t1882 * t7276;
    let t32488 = t1851 * t7274;
    let t32489 = t32488 * t379;
    (t32475, t32479, t32483, t32487, t32488, t32489)
}
