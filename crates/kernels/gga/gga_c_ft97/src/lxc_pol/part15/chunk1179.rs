//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1179/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1179<F: Float>(t1701: F, t22059: F, t3780: F, t1208: F, t14722: F, t83158: F, t1111: F, t1196: F, t14729: F, t22071: F, t22154: F, t2691: F, t285: F, t287: F, t290: F, t4099: F, t4113: F, t5003: F, t5016: F, t5231: F, t5261: F, t5262: F, t52752: F, t5285: F, t65952: F, t79714: F, t79757: F, t79759: F, t79782: F, t817: F, t82816: F, t82851: F, t82855: F, t88542: F, t88544: F, t88548: F, t88552: F, t88556: F, t88560: F, t88562: F, t88566: F, t88570: F, t88572: F, t88608: F, t88610: F, t88614: F, t88617: F, t88881: F, t90124: F, t90147: F) -> (F, F, F) {
    let t90153 = t1701 * t3780 * t22059;
    let t90159 = t14722 * t83158 * t1208;
    let t90168 = -F::cast_from(0.35032929183548774394e2_f64) * t22154 * t5016 + F::cast_from(0.17516464591774387197e2_f64) * t5262 * t5016 - F::cast_from(0.14498192132169191472e2_f64) * t82816 * t5261 * t1111 + F::cast_from(24.0_f64) * t5231 * t5285 - F::cast_from(0.61919070671564293155e1_f64) * t22071 * t88881 * t287 * t290 - t285 * t817 * (-F::cast_from(0.17780800291358024692e0_f64) * t88542 - F::cast_from(0.55318045350891632375e0_f64) * t88544 + F::cast_from(0.80013601311111111114e0_f64) * t88548 + F::cast_from(0.66678001092592592595e-1_f64) * t88552 + F::cast_from(0.17286889172153635117e0_f64) * t88556 + F::cast_from(0.16669500273148148149e-1_f64) * t88560 + F::cast_from(0.14224640233086419754e1_f64) * t88562 - F::cast_from(0.40006800655555555556e0_f64) * t88566 - F::cast_from(0.10001700163888888889e0_f64) * t88570 - F::cast_from(0.19558880320493827161e1_f64) * t88572 + t90124 + F::cast_from(0.2469555596021947874e-1_f64) * t52752 - F::cast_from(0.1333560021851851852e0_f64) * t79714 - F::cast_from(0.65196267734979423872e0_f64) * t65952 + F::cast_from(0.22226000364197530865e-1_f64) * t79757 + F::cast_from(0.35561600582716049384e0_f64) * t79759 + F::cast_from(0.88904001456790123462e-1_f64) * t79782 + F::cast_from(0.60010200983333333334e0_f64) * t88608 - F::cast_from(0.71123201165432098768e0_f64) * t88610 + F::cast_from(0.31116400509876543211e0_f64) * t88614 - F::cast_from(0.13335600218518518519e0_f64) * t88617 + t90147) + F::cast_from(0.2416365355361531912e1_f64) * t4099 * t90153 - F::cast_from(0.45910941751869106328e2_f64) * t5262 * t5003 + F::cast_from(0.14498192132169191472e2_f64) * t14729 * t90159 - F::cast_from(8.0_f64) * t2691 * t82851 * t1196 + F::cast_from(8.0_f64) * t4113 * t82855 * t1208;
    (t90153, t90159, t90168)
}
