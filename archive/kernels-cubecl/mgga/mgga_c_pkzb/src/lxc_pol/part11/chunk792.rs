//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 792/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk792<F: Float>(t1123: F, t785: F, t2019: F, t2916: F, t306: F, t2968: F, t5718: F, t1133: F, t2956: F, t751: F, t2036: F, t1429: F) -> (F, F, F, F, F, F, F) {
    let t7836 = t785 * t1123;
    let t7837 = t2019 * t7836;
    let t7840 = t306 * t2916;
    let t7841 = t2019 * t7840;
    let t7844 = t5718 * t2968;
    let t7871 = t2019 * t1133;
    let t7874 = t751 * t2956;
    let t7879 = t2036 * t1133;
    let t7906 = F::cast_from(2.0_f64) * t1429;
    (t7837, t7841, t7844, t7871, t7874, t7879, t7906)
}
