//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1042/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1042<F: Float>(t1556: F, t8396: F, t6505: F, t6581: F, t6204: F, t8307: F, t1308: F, t3973: F, t8327: F, t1580: F, t1056: F, t2326: F, t5670: F, t21501: F, t1591: F, t2075: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27694 = t8396 * t1556;
    let t27701 = t6505 * t6581;
    let t27702 = t6204 * t27701;
    let t27705 = t8307 * sigma0;
    let t27706 = t27705 * t1308;
    let t27709 = t3973 * t8327;
    let t27710 = t1580 * t27709;
    let t27720 = t2326 * t1056;
    let t27721 = t5670 * t27720;
    let t27722 = t21501 * t27721;
    let t27725 = t2326 * t1591;
    let t27726 = t2075 * t27725;
    (t27694, t27702, t27706, t27710, t27720, t27721, t27722, t27725, t27726)
}
