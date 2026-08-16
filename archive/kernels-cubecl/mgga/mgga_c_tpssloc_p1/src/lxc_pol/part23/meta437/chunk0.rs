//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1279/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1279<F: Float>(t11570: F, t20234: F, t18457: F, t4889: F, t18321: F, t4896: F, t18451: F, t1174: F, t22081: F, t44562: F, t22046: F, t3431: F) -> (F, F, F, F, F, F) {
    let t73225 = t11570 * t20234;
    let t73272 = t4889 * t18457;
    let t73274 = t18321 * t4896;
    let t73276 = t4889 * t18451;
    let t73279 = t1174 * t44562 * t22081;
    let t73287 = t1174 * t3431 * t22046;
    (t73225, t73272, t73274, t73276, t73279, t73287)
}
