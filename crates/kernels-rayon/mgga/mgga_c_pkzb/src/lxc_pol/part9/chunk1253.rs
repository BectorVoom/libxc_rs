//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1253/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1253(t735: f64, t7628: f64, t154: f64, t2739: f64, t276: f64, t5688: f64, t1120: f64, t18185: f64, t18202: f64, t18207: f64, t18216: f64, t18218: f64, t18232: f64, t18234: f64, t18236: f64, t1843: f64, t2057: f64, t2895: f64, t2945: f64, t5592: f64, t758: f64, t7787: f64) -> f64 {
    let t21946 = t735 * t7628;
    let t21950 = t276 * t154 * t5688 * t2739;
    let t21951 = t21950 / 144.0_f64;
    let t21960 = -t18185 / 48.0_f64 - 0.85748036236139473943e-3_f64 * t18202 - 0.34299214494455789578e-2_f64 * t18207 - 0.13719685797782315831e-1_f64 * t18216 + 0.45732285992607719436e-2_f64 * t18218 - 0.28582678745379824648e-3_f64 * t18232 + 77.0_f64 / 162.0_f64 * t5592 * t1120 + t21946 / 18.0_f64 + t21951 - 11.0_f64 / 36.0_f64 * t2057 * t2895 - 0.28963781128651555643e-1_f64 * t18234 - 0.30488190661738479624e-2_f64 * t18236 + 0.38586616306262763276e-2_f64 * t2945 * t758 * t7787 * t1843;
    t21960
}
