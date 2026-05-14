//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 609/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk609<F: Float>(t673: F, t6943: F, t716: F, t720: F, t415: F, t2538: F, t4811: F, t2529: F, t2528: F, t4817: F, t1869: F, t4809: F, t5080: F, t6664: F, t6670: F, t6678: F, t6682: F, t6687: F, t6692: F, t6695: F, t6700: F, t6705: F, t6710: F, t6717: F, t6721: F, t6725: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6944 = t673 * t6943;
    let t6945 = t6944 * t716;
    let t6946 = t6945 * t720;
    let t6947 = t415 * t6946;
    let t6949 = t4811 * t2538;
    let t6951 = t4811 * t2529;
    let t6953 = t4817 * t2528;
    let t6954 = t1869 * t6953;
    let t6956 = 0.11054629629629629629e-2 * t6664 - 0.33163888888888888888e-2 * t6670 + 0.27636574074074074073e-2 * t6678 - 0.16581944444444444444e-2 * t6682 + 0.49745833333333333332e-2 * t6687 - 0.16581944444444444444e-2 * t6692 + 0.16581944444444444444e-2 * t6695 - 0.44218518518518518517e-2 * t6700 - 0.16581944444444444444e-2 * t6705 - 0.55273148148148148147e-3 * t6710 - 0.33163888888888888888e-2 * t6717 + 0.16581944444444444444e-2 * t6721 - t4809 + 0.16581944444444444444e-2 * t5080 - 0.66327777777777777776e-2 * t6725 + 0.24872916666666666666e-2 * t6947 + 0.11054629629629629629e-2 * t6949 - 0.16581944444444444444e-2 * t6951 - 0.24872916666666666666e-2 * t6954;
    (t6944, t6945, t6946, t6947, t6949, t6951, t6953, t6954, t6956)
}
