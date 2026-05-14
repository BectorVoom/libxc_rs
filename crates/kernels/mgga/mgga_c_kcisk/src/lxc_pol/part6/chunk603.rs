//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 603/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk603<F: Float>(t1899: F, t8946: F, t1873: F, t1869: F, t2473: F, t6719: F, t1799: F, t1801: F, t8518: F, t1800: F, t8510: F, t5054: F, t671: F, t6990: F, t6992: F, t8616: F, t8868: F, t8872: F, t8876: F, t8880: F, t8884: F, t8887: F, t8890: F, t8942: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8947 = t1899 * t8946;
    let t8948 = t1873 * t8947;
    let t8949 = t1869 * t8948;
    let t8951 = t6719 * t2473;
    let t8952 = t1799 * t8951;
    let t8954 = t1801 * t8518;
    let t8955 = t1800 * t8954;
    let t8956 = t1799 * t8955;
    let t8958 = t1801 * t8510;
    let t8959 = t1800 * t8958;
    let t8960 = t5054 * t8959;
    let t8963 = -0.88437037037037037034e-2 * t8868 - 0.33163888888888888888e-2 * t8872 - 0.55273148148148148147e-3 * t8876 - 0.88437037037037037034e-2 * t8880 + 0.16581944444444444444e-2 * t8884 - 0.49745833333333333332e-2 * t8887 + 0.33163888888888888888e-2 * t8890 - 0.24872916666666666666e-2 * t8942 - 0.88437037037037037034e-2 * t6990 + 0.33163888888888888888e-2 * t6992 - 0.33163888888888888888e-2 * t8949 + 0.33163888888888888888e-2 * t8952 + 0.16581944444444444444e-2 * t8956 + 0.27636574074074074073e-2 * t8960 + t8616 * t671;
    (t8947, t8948, t8949, t8951, t8952, t8954, t8955, t8956, t8958, t8959, t8960, t8963)
}
