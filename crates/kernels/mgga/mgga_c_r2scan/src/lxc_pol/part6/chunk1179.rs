//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1179/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1179<F: Float>(t21478: F, t219: F, t225: F, t1821: F, t21115: F, t5290: F, t1835: F, t1956: F, t1973: F, t1981: F, t1982: F, t1986: F, t201: F, t207: F, t208: F, t21244: F, t21416: F, t21430: F, t21467: F, t21472: F, t5270: F, t5507: F, t5524: F, t5549: F, t664: F, t674: F, t682: F, t718: F, t721: F) -> (F, F, F, F) {
    let t21480 = t219 * t21478 * t225;
    let t21483 = t5290 * t21115 * t1821;
    let t21509 = t1835 * t21115 * t225;
    let t21511 = 0.20548e0 * t201 * t21467 * t207 + 0.10132939716376971859e5 * t21472 + 0.65061487801810439052e-1 * t21480 - 0.13689115175718902887e4 * t21483 + 0.739728e1 * t1973 * t5507 - 8.0 * t674 * t5524 * t664 - 8.0 * t674 * t682 * t5549 - 2.0 * t674 * t208 * t21467 + 0.24934837112181307812e4 * t1981 * t1986 * t5270 - t21244 + 0.30762056574649219974e4 * t1981 * t1982 * t21430 + 0.17315859105681463759e2 * t718 * t721 * t21478 - 0.493152e1 * t1956 * t21416 * t207 - 0.15614757072434505372e1 * t21509;
    (t21480, t21483, t21509, t21511)
}
