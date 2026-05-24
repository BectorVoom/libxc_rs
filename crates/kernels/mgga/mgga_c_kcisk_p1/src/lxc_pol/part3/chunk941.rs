//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 941/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk941<F: Float>(t13803: F, t13849: F, t13889: F, t13932: F, t1328: F, t3579: F, t3764: F, t1340: F, t1339: F, t1440: F, t3732: F, t1341: F) -> (F, F, F, F) {
    let t13934 = t13803 + t13849 + t13889 + t13932;
    let t13935 = t13934 * t1328;
    let t13938 = t3764 * t3579;
    let t13939 = t1340 * t13938;
    let t13940 = t1339 * t13939;
    let t13944 = t3732 * t1440;
    let t13945 = t1341 * t13944;
    (t13935, t13940, t13944, t13945)
}
