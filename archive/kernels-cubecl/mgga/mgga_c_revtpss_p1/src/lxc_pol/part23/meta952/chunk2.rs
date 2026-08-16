//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3157/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3157<F: Float>(t17569: F, t20783: F, t1042: F, t1261: F, t12866: F, t17693: F, t17694: F, t20820: F, t5268: F, t5287: F, t69936: F, t69939: F, t69947: F, t69961: F, t69964: F, t69966: F, t78770: F, t82587: F, t82591: F) -> F {
    let t82932 = t17569 * t20783;
    let t82950 = F::cast_from(0.28582678745379824648e-3_f64) * t69936 + F::cast_from(0.57165357490759649295e-3_f64) * t69939 + F::cast_from(0.57165357490759649296e-3_f64) * t82932 - F::cast_from(0.30488190661738479624e-2_f64) * t69947 - F::cast_from(0.7145669686344956162e-3_f64) * t12866 * t17694 * t82591 + F::cast_from(0.71456696863449561621e-3_f64) * t17693 * t17694 * t82587 + F::cast_from(0.64311027177104605458e-3_f64) * t20820 * t5287 - F::cast_from(0.5081365110289746604e-2_f64) * t69961 + F::cast_from(0.14291339372689912324e-3_f64) * t69964 + F::cast_from(0.57165357490759649296e-3_f64) * t69966 - F::cast_from(0.28582678745379824648e-3_f64) * t1261 * t1042 * t5268 * t78770;
    t82950
}
