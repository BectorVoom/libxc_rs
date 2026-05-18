//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 657/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk657<F: Float>(t10942: F, t9800: F, t10905: F, t10908: F, t10911: F, t10918: F, t10921: F, t10923: F, t10927: F, t10934: F, t10935: F, t10937: F, t10941: F, t9935: F, t9937: F, t9942: F, t9946: F) -> F {
    let t10943 = t9800 * t10942;
    let t10944 = F::new(0.9585731488480187419e0) * t10943;
    let t10945 = t10905 - t10908 + t10911 - t10918 + t10921 + t10923 - t10927 + t10934 + t9935 + t9937 - t9942 - t9946 + t10935 + t10937 - t10941 + t10944;
    t10945
}
