//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1049/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1049<F: Float>(t1163: F, t1181: F, t4289: F, t5725: F, t4396: F, t5743: F, t1532: F, t322: F, t5799: F, t1524: F, t944: F, t1165: F, t1531: F, t1552: F, t16057: F, t16072: F, t16083: F, t16110: F, t16117: F, t3396: F, t406: F, t4263: F, t4298: F, t5740: F, t5741: F, t6337: F, t929: F) -> (F,) {
    let t21060 = t1163 * t1181 * t4289 * t5725;
    let t21066 = t4396 * t5743;
    let t21071 = t1163 * t1181 * t1532 * t5799 * t322;
    let t21077 = t944 * t1524;
    let t21093 = 0.17149607247227894789e-2 * t21060 - 0.41159057393346947494e-1 * t3396 * t1181 * t6337 * t4263 - 0.34299214494455789578e-2 * t21066 + 0.17149607247227894789e-2 * t21071 - 0.34299214494455789578e-2 * t1531 * t1165 * t4298 * t5741 - 0.34299214494455789578e-2 * t1531 * t1165 * t1552 * t21077 * t406 - 0.17149607247227894789e-2 * t1531 * t1165 * t1552 * t5740 * t929 + 0.24009450146119052705e0 * t16057 - 0.17149607247227894789e-2 * t16072 + 0.16006300097412701803e-1 * t16083 + 0.42874018118069736972e-3 * t16110 + 0.68598428988911579156e-2 * t16117;
    (t21093,)
}
