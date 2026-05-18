//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1021/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1021<F: Float>(t40184: F, t41851: F, t41852: F, t41854: F, t41874: F, t41885: F, t41889: F, t41893: F, t41904: F, t41905: F, t41907: F, t47925: F, t47926: F, t47927: F, t47934: F, t47937: F, t47939: F, t47941: F, t47949: F, t47951: F) -> F {
    let t50858 = -t41851 + t41852 + F::new(0.89376224879626066675e-1) * t40184 - t41854 - t47925 + t47926 - F::new(0.21450293971110256002e1) * t47927 + t41874 + F::new(0.71500979903700853338e0) * t47934 + F::new(0.71500979903700853338e0) * t47937 + F::new(0.71500979903700853338e0) * t47939 - F::new(0.14300195980740170668e1) * t47941 + t41885 - t41889 + t41893 + t41904 - t41905 + t41907 - F::new(0.38342925953920749676e0) * t47949 + F::new(0.71500979903700853338e0) * t47951;
    t50858
}
