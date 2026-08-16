//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1085/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1085(t31233: f64, t31235: f64, t31237: f64, t31239: f64, t31883: f64, t31885: f64, t31887: f64, t32595: f64, t32609: f64, t671: f64, t8446: f64, t1393: f64, t31055: f64, t31057: f64, t31060: f64, t31077: f64, t31088: f64, t31089: f64, t31223: f64, t31249: f64, t31898: f64, t31900: f64, t31902: f64, t31904: f64, t31906: f64, t31909: f64, t31916: f64, t31919: f64, t574: f64, t672: f64, t8916: f64) -> (f64, f64) {
    let t32623 = 2.0_f64 * t32609 * t671 + t31233 + t31235 + t31237 + t31239 + 4.0_f64 * t31883 + 4.0_f64 * t31885 + 4.0_f64 * t31887 + t32595 + t8446;
    let t32628 = t1393 * t8916 - 2.0_f64 * t32609 * t672 + t32623 * t574 - t31055 - t31057 - t31060 - t31077 - t31088 + t31089 + t31223 - t31249 - 4.0_f64 * t31898 - 4.0_f64 * t31900 - 4.0_f64 * t31902 - 4.0_f64 * t31904 - 4.0_f64 * t31906 - 4.0_f64 * t31909 + 6.0_f64 * t31916 - 4.0_f64 * t31919;
    (t32623, t32628)
}
