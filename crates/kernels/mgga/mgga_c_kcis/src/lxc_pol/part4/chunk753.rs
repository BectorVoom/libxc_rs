//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 753/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk753<F: Float>(t1096: F, t5019: F, t1092: F, t1775: F, t2861: F, t1094: F, t1747: F, t1122: F, t1134: F, t4999: F, t1010: F, t1710: F, t300: F, t3049: F, t3247: F, t3248: F, t4768: F, t4926: F, t4978: F, t4981: F, t4987: F, t4990: F, t4997: F, t5001: F, t5003: F, t5007: F, t5011: F, t5015: F, t5017: F, t979: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5020 = t1096 * t5019;
    let t5021 = t1092 * t5020;
    let t5023 = t2861 * t1775;
    let t5025 = t1747 * t1094;
    let t5026 = t5025 * sigma0;
    let t5027 = t5026 * t1122;
    let t5028 = t1092 * t5027;
    let t5030 = t4999 * t1134;
    let t5031 = t1092 * t5030;
    let t5033 = 0.24872916666666666666e-2 * t4926 - t3247 - 0.44218518518518518517e-2 * t3248 - 0.66725e-1 * t3049 * t1710 - 0.66725e-1 * t979 * t4978 - 0.66725e-1 * t4981 * t1010 - 0.16581944444444444444e-2 * t4987 + 0.16581944444444444444e-2 * t4990 + 0.33163888888888888888e-2 * t4997 + 0.16581944444444444444e-2 * t5001 + 0.11054629629629629629e-2 * t5003 - 0.44218518518518518517e-2 * t5007 + t4768 * t300 + 0.16581944444444444444e-2 * t5011 - 0.44218518518518518517e-2 * t5015 - 0.16581944444444444444e-2 * t5017 + 0.66327777777777777776e-2 * t5021 + 0.11054629629629629629e-2 * t5023 - 0.24872916666666666666e-2 * t5028 + 0.16581944444444444444e-2 * t5031;
    (t5020, t5021, t5023, t5025, t5026, t5027, t5028, t5030, t5031, t5033)
}
