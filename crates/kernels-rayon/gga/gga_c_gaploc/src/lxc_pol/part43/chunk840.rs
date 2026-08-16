//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 840/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk840(t41810: f64, t6710: f64, t6711: f64, t2877: f64, t9490: f64, t9494: f64, t40172: f64, t40176: f64, t40178: f64, t40182: f64, t40187: f64, t10557: f64, t9324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41837 = 0.11502877786176224903e2_f64 * t6710 * t6711 * t41810;
    let t41844 = 0.35750489951850426669e0_f64 * t9490 * t2877;
    let t41846 = 0.35750489951850426669e0_f64 * t9494 * t2877;
    let t41849 = 0.19171462976960374838e1_f64 * t40172;
    let t41850 = 0.42603251059911944084e0_f64 * t40176;
    let t41851 = 0.11502877786176224903e1_f64 * t40178;
    let t41852 = 0.25561950635947166451e0_f64 * t40182;
    let t41854 = 0.17875244975925213335e0_f64 * t40187;
    let t41874 = 0.85801175884441024006e1_f64 * t10557 * t9324;
    (t41837, t41844, t41846, t41849, t41850, t41851, t41852, t41854, t41874)
}
