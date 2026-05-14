//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 706/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk706<F: Float>(t10791: F, t1248: F, t2364: F, t2404: F, t4857: F, t4908: F, t2541: F, t5217: F, t17056: F, t740: F, t5320: F, t6973: F, t718: F, t7336: F, t1934: F, t2532: F) -> (F, F, F, F, F, F, F, F) {
    let t17385 = t1248 * t10791 * t2364;
    let t17520 = t2404 * t4857;
    let t17567 = t2404 * t4908;
    let t17775 = t2541 * t5217;
    let t17821 = t17056 * t740;
    let t17933 = t6973 * t5320;
    let t17936 = t7336 * t718;
    let t17969 = t1934 * t2532;
    (t17385, t17520, t17567, t17775, t17821, t17933, t17936, t17969)
}
