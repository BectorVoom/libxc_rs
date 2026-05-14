//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1242/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1242<F: Float>(t10708: F, t1852: F, t10703: F, t10723: F, t8225: F, t10719: F, t10621: F, t10706: F, t10717: F, t2013: F, t2033: F, t2039: F, t22055: F, t22057: F, t22059: F, t22070: F, t22100: F, t25831: F, t3188: F, t3194: F, t3979: F, t571: F, t674: F) -> (F,) {
    let t30369 = t1852 * t10708;
    let t30371 = t1852 * t10703;
    let t30377 = t8225 * t10723;
    let t30383 = t1852 * t10719;
    let t30390 = -40.0 / 729.0 * t571 * t25831 * t22070 * t3979 * t2039 + 2.0 / 27.0 * t571 * t3188 * t10706 * t2013 - 4.0 / 81.0 * t30369 + 10.0 / 729.0 * t30371 + 28.0 / 729.0 * t22055 - 2.0 / 243.0 * t22057 - 4.0 / 729.0 * t22059 + 4.0 / 243.0 * t22100 + 44.0 / 81.0 * t30377 - t571 * t3194 * t10717 * t2013 / 9.0 + 2.0 / 27.0 * t30383 + 2.0 / 27.0 * t571 * t3194 * t2033 * t10621 * t674;
    (t30390,)
}
