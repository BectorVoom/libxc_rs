//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1038/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1038<F: Float>(t1285: F, t1294: F, t4509: F, t1231: F, t1253: F, t1258: F, t1304: F, t1305: F, t1314: F, t1320: F, t1322: F, t18442: F, t18563: F, t18619: F, t18629: F, t18639: F, t18655: F, t18658: F, t18667: F, t395: F, t4: F, t440: F, t455: F, t4573: F, t4620: F, t4643: F, t4679: F, t4681: F, t4735: F, t4801: F, t4805: F, t71: F, t84: F) -> (F, F) {
    let t18838 = F::cast_from(0.57894567559743977359e3_f64) * t4509 * t1294 * t1285;
    let t18839 = F::cast_from(0.21053604230838734656e2_f64) * t1320 * t1305 * t1314 + t18619 - F::cast_from(0.66091990993888710196e1_f64) * t395 * t1258 * t440 * t4643 - F::cast_from(0.1926377843805564792e1_f64) * t395 * t4801 + F::cast_from(0.13012297059337829057e0_f64) * t395 * t4805 + t18629 - F::cast_from(0.55208163456790123453e-2_f64) * t4 * t4573 * t71 - t18655 - t18658 + t18667 - F::cast_from(0.46785787179641632568e1_f64) * t1304 * t4620 * t455 + F::cast_from(0.51947267698127589897e2_f64) * t1320 * t18563 * t1322 + F::cast_from(0.6233672123775310788e3_f64) * t4735 * t18639 * t1322 + F::cast_from(0.12414802127193579148e5_f64) * t4679 * t1231 * t4681 * t1253 - F::cast_from(0.18989760778855128827e-2_f64) * t4 * t4573 * t84 + F::cast_from(0.69263023597503453196e2_f64) * t1320 * t18442 * t455 + t18838;
    (t18838, t18839)
}
