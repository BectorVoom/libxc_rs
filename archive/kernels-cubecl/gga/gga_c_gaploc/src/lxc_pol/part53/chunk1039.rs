//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1039/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1039<F: Float>(t42857: F, t42863: F, t42866: F, t42867: F, t42868: F, t42869: F, t42870: F, t42871: F, t42872: F, t42873: F, t42874: F, t42877: F, t42881: F, t42893: F, t42896: F, t42899: F, t47050: F, t47054: F) -> F {
    let t50979 = t42857 + t42863 + t42866 - t42867 + t42868 + F::cast_from(0.17073003981405689759e0_f64) * t47050 - t42869 - t42870 - t42871 + t42872 + t42873 + t42874 + t47054 - t42877 + t42881 - t42893 + t42896 + t42899;
    t50979
}
