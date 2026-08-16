//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1038/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1038(t1285: f64, t1294: f64, t4509: f64, t1231: f64, t1253: f64, t1258: f64, t1304: f64, t1305: f64, t1314: f64, t1320: f64, t1322: f64, t18442: f64, t18563: f64, t18619: f64, t18629: f64, t18639: f64, t18655: f64, t18658: f64, t18667: f64, t395: f64, t4: f64, t440: f64, t455: f64, t4573: f64, t4620: f64, t4643: f64, t4679: f64, t4681: f64, t4735: f64, t4801: f64, t4805: f64, t71: f64, t84: f64) -> (f64, f64) {
    let t18838 = 0.57894567559743977359e3_f64 * t4509 * t1294 * t1285;
    let t18839 = 0.21053604230838734656e2_f64 * t1320 * t1305 * t1314 + t18619 - 0.66091990993888710196e1_f64 * t395 * t1258 * t440 * t4643 - 0.1926377843805564792e1_f64 * t395 * t4801 + 0.13012297059337829057e0_f64 * t395 * t4805 + t18629 - 0.55208163456790123453e-2_f64 * t4 * t4573 * t71 - t18655 - t18658 + t18667 - 0.46785787179641632568e1_f64 * t1304 * t4620 * t455 + 0.51947267698127589897e2_f64 * t1320 * t18563 * t1322 + 0.6233672123775310788e3_f64 * t4735 * t18639 * t1322 + 0.12414802127193579148e5_f64 * t4679 * t1231 * t4681 * t1253 - 0.18989760778855128827e-2_f64 * t4 * t4573 * t84 + 0.69263023597503453196e2_f64 * t1320 * t18442 * t455 + t18838;
    (t18838, t18839)
}
