//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1111/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1111<F: Float>(t1125: F, t17955: F, t757: F, t2096: F, t7581: F, t154: F, t2739: F, t276: F, t5688: F, t17938: F, t18290: F, t2019: F, t2956: F) -> (F, F, F, F, F) {
    let t21933 = t757 * t17955 * t1125;
    let t21935 = t2096 * t7581;
    let t21950 = t276 * t154 * t5688 * t2739;
    let t21951 = t21950 / F::cast_from(144.0_f64);
    let t22007 = t17938 * t18290;
    let t22082 = t2019 * t2956;
    (t21933, t21935, t21951, t22007, t22082)
}
