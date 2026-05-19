//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 840/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk840<F: Float>(t41810: F, t6710: F, t6711: F, t2877: F, t9490: F, t9494: F, t40172: F, t40176: F, t40178: F, t40182: F, t40187: F, t10557: F, t9324: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41837 = F::cast_from(0.11502877786176224903e2_f64) * t6710 * t6711 * t41810;
    let t41844 = F::cast_from(0.35750489951850426669e0_f64) * t9490 * t2877;
    let t41846 = F::cast_from(0.35750489951850426669e0_f64) * t9494 * t2877;
    let t41849 = F::cast_from(0.19171462976960374838e1_f64) * t40172;
    let t41850 = F::cast_from(0.42603251059911944084e0_f64) * t40176;
    let t41851 = F::cast_from(0.11502877786176224903e1_f64) * t40178;
    let t41852 = F::cast_from(0.25561950635947166451e0_f64) * t40182;
    let t41854 = F::cast_from(0.17875244975925213335e0_f64) * t40187;
    let t41874 = F::cast_from(0.85801175884441024006e1_f64) * t10557 * t9324;
    (t41837, t41844, t41846, t41849, t41850, t41851, t41852, t41854, t41874)
}
