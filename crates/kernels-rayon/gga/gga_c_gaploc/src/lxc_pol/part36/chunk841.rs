//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 841/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk841(t41838: f64, t447: f64, t6963: f64, t6964: f64, t2877: f64, t9490: f64, t9494: f64, t40167: f64, t40170: f64, t40172: f64, t40176: f64, t40178: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41839 = t41838 * t447;
    let t41841 = t6963 * t6964 * t41839;
    let t41844 = 0.35750489951850426669e0_f64 * t9490 * t2877;
    let t41846 = 0.35750489951850426669e0_f64 * t9494 * t2877;
    let t41847 = 0.3575048995185042667e0_f64 * t40167;
    let t41848 = 0.17875244975925213335e0_f64 * t40170;
    let t41849 = 0.19171462976960374838e1_f64 * t40172;
    let t41850 = 0.42603251059911944084e0_f64 * t40176;
    let t41851 = 0.11502877786176224903e1_f64 * t40178;
    (t41839, t41841, t41844, t41846, t41847, t41848, t41849, t41850, t41851)
}
