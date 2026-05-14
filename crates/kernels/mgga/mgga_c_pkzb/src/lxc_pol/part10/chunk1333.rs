//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1333/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1333<F: Float>(t2887: F, t68: F, t9554: F, t9297: F, t18202: F, t1843: F, t1885: F, t2104: F, t2105: F, t2888: F, t2889: F, t2976: F, t3645: F, t655: F, t7350: F, t7586: F, t779: F, t7857: F, t9161: F, t9296: F, t9298: F, t9302: F, t9553: F, t9555: F, t9575: F, t9589: F, t9594: F) -> (F,) {
    let t26585 = t2887 * t68 * t9554;
    let t26588 = t2887 * t68 * t9297;
    let t26590 = -0.85748036236139473944e-3 * t2104 * t2105 * t7857 * t3645 - 0.17149607247227894789e-2 * t2104 * t2105 * t2976 * t9575 - 0.28582678745379824648e-3 * t18202 - t2887 * t2888 * t9589 * t1885 / 16.0 + t2887 * t2888 * t9594 * t1885 / 4.0 + t2887 * t2888 * t779 * t9161 * t655 / 24.0 + t2887 * t2888 * t9553 * t1843 / 48.0 - t2887 * t2888 * t9296 * t1843 / 16.0 + t2887 * t2888 * t2889 * t7350 / 24.0 - t7586 * t9555 / 9.0 + t7586 * t9298 / 3.0 - 2.0 / 9.0 * t7586 * t9302 + t26585 / 72.0 - t26588 / 24.0;
    (t26590,)
}
