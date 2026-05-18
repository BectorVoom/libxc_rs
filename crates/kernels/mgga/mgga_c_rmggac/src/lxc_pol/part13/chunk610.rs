//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 610/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk610<F: Float>(t678: F, t7944: F, t7188: F, t2208: F, t4041: F, t7195: F, t7201: F, t7207: F, t7218: F, t7222: F, t7226: F, t7235: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7945 = t7944 * t678;
    let t8013 = F::new(0.5987120850931904282e-1) * t7188;
    let t8014 = t4041 * t2208;
    let t8015 = F::new(0.11974241701863808564e0) * t8014;
    let t8016 = F::new(0.5454932330849068346e-1) * t7195;
    let t8017 = F::new(0.16364796992547205038e0) * t7201;
    let t8018 = F::new(0.40911992481368012596e-1) * t7207;
    let t8021 = F::new(0.30487649791575028312e-3) * t7218;
    let t8022 = F::new(0.60975299583150056624e-3) * t7222;
    let t8023 = F::new(0.30487649791575028312e-3) * t7226;
    let t8024 = F::new(0.212822999466489197e-4) * t7235;
    (t7945, t8013, t8015, t8016, t8017, t8018, t8021, t8022, t8023, t8024)
}
