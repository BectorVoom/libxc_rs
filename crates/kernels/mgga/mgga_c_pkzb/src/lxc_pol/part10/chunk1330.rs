//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1330/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1330<F: Float>(t2099: F, t757: F, t9541: F, t17848: F, t2104: F, t9288: F, t5974: F, t9558: F, t2899: F, t774: F, t9563: F, t179: F, t2068: F, t299: F, t9161: F, t18110: F, t18114: F, t18123: F, t1843: F, t21882: F, t21928: F, t25315: F, t2945: F, t5984: F, t758: F, t771: F, t780: F, t9253: F, t9270: F, t9289: F, t9589: F) -> (F,) {
    let t26413 = t757 * t2099 * t9541;
    let t26423 = t2104 * t17848 * t9288;
    let t26426 = t2104 * t5974 * t9558;
    let t26430 = t2899 * t774 * t9563;
    let t26440 = t299 * t179 * t2068 * t9161;
    let t26447 = 0.28582678745379824648e-3 * t26413 - 0.68598428988911579156e-2 * t21882 - 0.2540682555144873302e-3 * t18110 + 0.95275595817932748826e-4 * t18114 + 0.45732285992607719436e-2 * t5984 * t9270 - 0.13719685797782315831e-1 * t5984 * t9289 + 0.17149607247227894789e-2 * t26423 - 0.57165357490759649296e-3 * t26426 - 11.0 / 486.0 * t18123 + 0.57165357490759649296e-3 * t26430 - 0.42874018118069736972e-3 * t299 * t179 * t780 * t25315 + 0.45732285992607719436e-2 * t771 * t9253 - 0.57165357490759649296e-3 * t26440 + 0.28582678745379824648e-3 * t21928 + 0.12862205435420921092e-2 * t2945 * t758 * t9589 * t1843;
    (t26447,)
}
