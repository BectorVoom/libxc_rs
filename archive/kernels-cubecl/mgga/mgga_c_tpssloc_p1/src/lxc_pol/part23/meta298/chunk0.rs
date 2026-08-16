//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1024/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1024<F: Float>(t21444: F, t340: F, t343: F, t974: F, t1597: F, t5836: F, t4546: F, t5842: F, t20217: F, t978: F, t977: F, t10217: F, t20234: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21446 = t340 * t21444 * t343;
    let t21447 = t974 * t21446;
    let t21452 = t5836 * t1597 * t343;
    let t21453 = t4546 * t21452;
    let t21456 = t5842 * t1597;
    let t21458 = t340 * t21456 * t343;
    let t21459 = t974 * t21458;
    let t21462 = t978 * t20217;
    let t21463 = t977 * t21462;
    let t21468 = t10217 * t20234;
    (t21446, t21447, t21452, t21453, t21456, t21458, t21459, t21462, t21463, t21468)
}
