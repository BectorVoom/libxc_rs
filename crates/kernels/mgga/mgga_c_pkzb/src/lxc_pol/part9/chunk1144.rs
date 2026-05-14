//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1144/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1144<F: Float>(t735: F, t7628: F, t154: F, t2739: F, t276: F, t5688: F, t1120: F, t18185: F, t18202: F, t18207: F, t18216: F, t18218: F, t18232: F, t18234: F, t18236: F, t1843: F, t2057: F, t2895: F, t2945: F, t5592: F, t758: F, t7787: F) -> (F,) {
    let t21946 = t735 * t7628;
    let t21950 = t276 * t154 * t5688 * t2739;
    let t21951 = t21950 / 144.0;
    let t21960 = -t18185 / 48.0 - 0.85748036236139473943e-3 * t18202 - 0.34299214494455789578e-2 * t18207 - 0.13719685797782315831e-1 * t18216 + 0.45732285992607719436e-2 * t18218 - 0.28582678745379824648e-3 * t18232 + 77.0 / 162.0 * t5592 * t1120 + t21946 / 18.0 + t21951 - 11.0 / 36.0 * t2057 * t2895 - 0.28963781128651555643e-1 * t18234 - 0.30488190661738479624e-2 * t18236 + 0.38586616306262763276e-2 * t2945 * t758 * t7787 * t1843;
    (t21960,)
}
