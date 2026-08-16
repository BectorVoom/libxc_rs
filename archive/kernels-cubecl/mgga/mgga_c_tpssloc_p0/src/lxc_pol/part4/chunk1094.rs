//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1094/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1094<F: Float>(t18047: F, t383: F, t4684: F, t5932: F, t3188: F, t4649: F, t1629: F, t4673: F, t1625: F, t1060: F, t1022: F, t5914: F) -> (F, F, F, F, F, F) {
    let t18129 = t383 * t18047;
    let t18131 = t5932 * t4684;
    let t18138 = t3188 * t4649;
    let t18139 = t1629 * t18138;
    let t18142 = t5932 * t4673;
    let t18150 = t1625 * t4649;
    let t18151 = t18150 * t1060;
    let t18154 = t5914 * t1022;
    (t18129, t18131, t18139, t18142, t18151, t18154)
}
