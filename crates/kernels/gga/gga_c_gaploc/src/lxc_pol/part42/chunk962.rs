//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 962/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk962<F: Float>(t209: F, t49834: F, t49851: F, t49859: F, t49891: F, t49907: F, t49942: F, t49958: F, t49961: F, t1382: F, t2902: F, t3718: F) -> (F, F) {
    let t49965 = (t49834 + t49851 + t49859 + t49891 + t49907 + t49942 + t49958 + t49961) * t209;
    let t49968 = F::new(4.0) * t1382 * t2902 * t3718;
    (t49965, t49968)
}
