//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 995/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk995<F: Float>(t3959: F, t6211: F, t1322: F, t6204: F, t1311: F, t3118: F, t6188: F, t1309: F, t13830: F, t2168: F, t3961: F, t25: F, t398: F) -> (F, F, F, F, F) {
    let t20140 = t3959 * t6211;
    let t20141 = t20140 * t1322;
    let t20142 = t6204 * t20141;
    let t20149 = t3118 * t1311;
    let t20150 = t20149 * t6188;
    let t20151 = t1309 * t20150;
    let t20154 = t13830 * t2168 * t3961;
    let t20155 = t6204 * t20154;
    let t20160 = t25 * t398;
    (t20142, t20149, t20151, t20155, t20160)
}
