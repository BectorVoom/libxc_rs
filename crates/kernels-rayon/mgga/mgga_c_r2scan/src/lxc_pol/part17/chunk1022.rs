//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1022/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1022(t12766: f64, t12782: f64, t12798: f64, t12809: f64, t797: f64, t1048: f64, t499: f64, t12033: f64, t3579: f64, t1044: f64, t3781: f64, t11206: f64, t11215: f64, t11866: f64, t11876: f64, t11886: f64, t12587: f64, t12589: f64, t12591: f64, t12593: f64, t12596: f64, t12599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12811 = t12766 + t12782 + t12798 + t12809;
    let t12812 = t12811 * t797;
    let t12814 = t1048 * t499 * t12812;
    let t12815 = t12814 / 4.0_f64;
    let t12816 = t3579 * t12033;
    let t12817 = t12816 / 2.0_f64;
    let t12818 = t3781 * t1044;
    let t12819 = 2.0_f64 * t12818;
    let t12828 = -t11206 - 4.0_f64 / 3.0_f64 * t11866 - t12587 / 2.0_f64 + t12589 / 4.0_f64 - t12591 / 4.0_f64 + t12593 + 4.0_f64 / 3.0_f64 * t11876 - 3.0_f64 / 2.0_f64 * t12596 - 8.0_f64 / 3.0_f64 * t11886 + t12599 / 2.0_f64 - t11215;
    (t12811, t12812, t12815, t12817, t12819, t12828)
}
