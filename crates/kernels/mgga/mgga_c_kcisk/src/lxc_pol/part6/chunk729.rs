//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 729/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk729<F: Float>(t4811: F, t8883: F, t8886: F, t8875: F, t8879: F, t8941: F, t1692: F, t8616: F, t8889: F, t5074: F, t8867: F, t8871: F, t8674: F, t8678: F, t8951: F, t1333: F, t8862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23874 = t4811 * t8883;
    let t23876 = t4811 * t8886;
    let t23878 = t4811 * t8875;
    let t23880 = t4811 * t8879;
    let t23894 = t4811 * t8941;
    let t23922 = t8616 * t1692;
    let t23947 = t4811 * t8889;
    let t23949 = t5074 * t8867;
    let t23951 = t5074 * t8871;
    let t23969 = t4811 * t8674;
    let t23976 = t4811 * t8678;
    let t23978 = t5074 * t8951;
    let t24073 = t1333 * t8862;
    (t23874, t23876, t23878, t23880, t23894, t23922, t23947, t23949, t23951, t23969, t23976, t23978, t24073)
}
