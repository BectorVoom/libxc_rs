//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1041/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1041<F: Float>(t209: F, t50939: F, t50949: F, t50953: F, t50958: F, t50962: F, t50966: F, t50977: F, t50979: F, t47107: F, t47114: F, t47120: F) -> (F, F, F, F) {
    let t50983 = (t50939 + t50949 + t50953 + t50958 + t50962 + t50966 + t50977 + t50979) * t209;
    let t50984 = F::cast_from(4.0_f64) * t47107;
    let t50985 = F::cast_from(4.0_f64) * t47114;
    let t50986 = F::cast_from(4.0_f64) * t47120;
    (t50983, t50984, t50985, t50986)
}
