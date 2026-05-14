//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1037/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1037<F: Float>(t4581: F, t8866: F, t1799: F, t11200: F, t8851: F, t1790: F, t11209: F, t16702: F, t1693: F, t17078: F, t23292: F, t23302: F, t23307: F, t23311: F, t23314: F, t23318: F, t23320: F, t23869: F, t23872: F, t23874: F, t23876: F, t23878: F, t23880: F, t4823: F, t7278: F, t7284: F) -> (F, F, F, F) {
    let t23882 = t4581 * t8866;
    let t23883 = t1799 * t23882;
    let t23885 = t8851 * t11200;
    let t23886 = t23885 * t1790;
    let t23892 = 0.148996e0 * t4823 * t23292 + 0.386e0 * t7278 * t7284 + 0.148996e0 * t17078 * t7284 - 0.16581944444444444444e-2 * t23302 - 0.55273148148148148147e-3 * t23307 - 0.22109259259259259259e-2 * t23311 - 0.33163888888888888888e-2 * t23314 - 0.7369753086419753086e-3 * t16702 + 0.27636574074074074073e-2 * t23318 + 0.16581944444444444444e-2 * t23320 - 0.193e0 * t1693 * t23869 - 0.22109259259259259259e-2 * t23872 + 0.11054629629629629629e-2 * t23874 - 0.33163888888888888888e-2 * t23876 - 0.36848765432098765431e-3 * t23878 - 0.58958024691358024689e-2 * t23880 - 0.88437037037037037034e-2 * t23883 - 0.386e0 * t1693 * t23886 - 0.223494e0 * t4823 * t23886 + 0.55273148148148148147e-3 * t11209;
    (t23883, t23885, t23886, t23892)
}
