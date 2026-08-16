//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 987/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk987(t3308: f64, t8066: f64, t574: f64, t2651: f64, t3309: f64, t10810: f64, t2608: f64, t10698: f64, t3588: f64, t10870: f64, t10886: f64, t10892: f64, t10895: f64, t10897: f64, t10902: f64, t10905: f64, t11835: f64) -> (f64, f64, f64, f64, f64) {
    let t11837 = t3308 * t8066;
    let t11838 = t574 * t11837;
    let t11840 = t2651 * t3309;
    let t11842 = t10810 * t2608;
    let t11843 = t574 * t11842;
    let t11845 = t10698 * t3588;
    let t11852 = -0.23287303101564395623e-1_f64 * t10870 - 0.43341108700271342816e-1_f64 * t11835 - 0.43341108700271342816e-1_f64 * t11838 - 0.43341108700271342816e-1_f64 * t11840 + 0.11557628986739024751e0_f64 * t11843 + 0.64025200389650807209e-1_f64 * t11845 + 0.11557628986739024751e0_f64 * t10886 + 0.34672886960217074253e0_f64 * t10892 + 0.27439371595564631661e-2_f64 * t10895 - 0.97574405393827830186e-2_f64 * t10897 - t10902 + 0.23287303101564395623e-1_f64 * t10905;
    (t11837, t11842, t11843, t11845, t11852)
}
