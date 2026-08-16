//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1118/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1118(t26991: f64, t27037: f64, t27058: f64, t27093: f64, t1281: f64, t7807: f64, t1291: f64, t7823: f64, t26889: f64, t26892: f64, t26894: f64, t26898: f64, t26900: f64, t26902: f64, t26904: f64, t26906: f64, t26908: f64, t26910: f64, t26912: f64, t26914: f64) -> (f64, f64, f64, f64) {
    let t27095 = t26991 + t27037 + t27058 + t27093;
    let t27100 = t7807 * t1281;
    let t27105 = t7823 * t1291;
    let t27120 = -0.4046875e-1_f64 * t26889 + 0.5e0_f64 * t26892 - 0.125e0_f64 * t26894 + 0.1875e0_f64 * t26898 - 0.26979166666666666667e-1_f64 * t26900 + 0.20234375e-1_f64 * t26902 + 0.21583333333333333334e0_f64 * t26904 - 0.53958333333333333334e-1_f64 * t26906 + 0.4046875e-1_f64 * t26908 + 0.28777777777777777778e0_f64 * t26910 - 0.68347222222222222224e0_f64 * t26912 - 0.89930555555555555557e-2_f64 * t26914;
    (t27095, t27100, t27105, t27120)
}
