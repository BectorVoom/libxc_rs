//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3152/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3152<F: Float>(t1263: F, t24616: F, t24633: F, t17525: F, t21188: F, t24758: F, t3172: F, t3711: F, t1261: F, t24643: F, t1042: F, t1122: F, t12809: F, t12956: F, t17344: F, t17448: F, t1774: F, t18281: F, t21028: F, t21219: F, t24649: F, t24751: F, t3720: F, t5245: F, t5296: F, t57571: F, t5825: F, t6640: F) -> F {
    let t82799 = t1263 * t24616;
    let t82816 = t1263 * t24633;
    let t82821 = t17525 * t21188;
    let t82824 = t3711 * t3172 * t24758;
    let t82827 = t1261 * t3172 * t24643;
    let t82831 = F::cast_from(0.64311027177104605458e-3_f64) * t12809 * t3720 * t24751 * t21028 - F::cast_from(0.42874018118069736972e-3_f64) * t17448 * t21219 + F::cast_from(0.85748036236139473947e-3_f64) * t17344 * t1042 * t82799 * t1122 + F::cast_from(0.42874018118069736972e-3_f64) * t12956 * t24649 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t5296 * t18281 * t1774 + F::cast_from(0.42874018118069736972e-3_f64) * t3711 * t1042 * t5296 * t5825 * t5245 + F::cast_from(0.14291339372689912324e-3_f64) * t3711 * t1042 * t82816 * t1122 - F::cast_from(0.45732285992607719436e-2_f64) * t82821 + F::cast_from(0.28582678745379824648e-3_f64) * t82824 + F::cast_from(0.47637797908966374414e-3_f64) * t82827 + F::cast_from(0.45732285992607719436e-2_f64) * t57571 * t6640;
    t82831
}
