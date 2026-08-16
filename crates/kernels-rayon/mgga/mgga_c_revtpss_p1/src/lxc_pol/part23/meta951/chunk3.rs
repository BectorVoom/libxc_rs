//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3152/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152(t1263: f64, t24616: f64, t24633: f64, t17525: f64, t21188: f64, t24758: f64, t3172: f64, t3711: f64, t1261: f64, t24643: f64, t1042: f64, t1122: f64, t12809: f64, t12956: f64, t17344: f64, t17448: f64, t1774: f64, t18281: f64, t21028: f64, t21219: f64, t24649: f64, t24751: f64, t3720: f64, t5245: f64, t5296: f64, t57571: f64, t5825: f64, t6640: f64) -> f64 {
    let t82799 = t1263 * t24616;
    let t82816 = t1263 * t24633;
    let t82821 = t17525 * t21188;
    let t82824 = t3711 * t3172 * t24758;
    let t82827 = t1261 * t3172 * t24643;
    let t82831 = 0.64311027177104605458e-3_f64 * t12809 * t3720 * t24751 * t21028 - 0.42874018118069736972e-3_f64 * t17448 * t21219 + 0.85748036236139473947e-3_f64 * t17344 * t1042 * t82799 * t1122 + 0.42874018118069736972e-3_f64 * t12956 * t24649 + 0.42874018118069736972e-3_f64 * t3711 * t1042 * t5296 * t18281 * t1774 + 0.42874018118069736972e-3_f64 * t3711 * t1042 * t5296 * t5825 * t5245 + 0.14291339372689912324e-3_f64 * t3711 * t1042 * t82816 * t1122 - 0.45732285992607719436e-2_f64 * t82821 + 0.28582678745379824648e-3_f64 * t82824 + 0.47637797908966374414e-3_f64 * t82827 + 0.45732285992607719436e-2_f64 * t57571 * t6640;
    t82831
}
