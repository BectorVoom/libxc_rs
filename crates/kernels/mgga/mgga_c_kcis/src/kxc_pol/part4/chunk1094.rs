//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1094/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1094<F: Float>(t15920: F, t4160: F, t11832: F, t11838: F, t15851: F, t15854: F, t15858: F, t15863: F, t15868: F, t15872: F, t15876: F, t15881: F, t15885: F, t15890: F, t15894: F, t15896: F, t15901: F, t15905: F, t15913: F, t15916: F) -> (F, F) {
    let t15921 = t4160 * t15920;
    let t15923 = 0.99491666666666666664e-2 * t15851 - 0.33163888888888888888e-2 * t15854 + 0.88437037037037037034e-2 * t15858 + 0.55273148148148148147e-3 * t15863 - 0.44218518518518518517e-2 * t15868 + 0.3684876543209876543e-2 * t15872 - 0.22109259259259259258e-2 * t15876 + 0.66327777777777777776e-2 * t15881 - 0.55273148148148148146e-2 * t15885 + 0.66327777777777777776e-2 * t15890 - 0.16581944444444444444e-2 * t11832 + 0.33163888888888888888e-2 * t15894 - 0.5895802469135802469e-2 * t15896 - 0.16581944444444444444e-2 * t15901 - 0.44218518518518518517e-2 * t15905 - 0.22109259259259259258e-2 * t11838 + 0.11054629629629629629e-2 * t15913 - 0.49745833333333333332e-2 * t15916 + 0.33163888888888888888e-2 * t15921;
    (t15921, t15923)
}
