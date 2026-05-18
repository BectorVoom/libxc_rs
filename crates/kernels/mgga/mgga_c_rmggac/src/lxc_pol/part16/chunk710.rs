//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 710/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk710<F: Float>(t9836: F, t2466: F, t4985: F, t1923: F, t2265: F, t9846: F, t9848: F, t9850: F, t9861: F, t9865: F, t9870: F, t9933: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10314 = F::new(0.13637330827122670865e-1) * t9836;
    let t10315 = t4985 * t2466;
    let t10316 = F::new(0.11974241701863808564e0) * t10315;
    let t10317 = t1923 * t2265;
    let t10318 = F::new(0.2363e1) * t10317;
    let t10319 = F::new(0.212822999466489197e-4) * t9846;
    let t10320 = F::new(0.1702583995731913576e-4) * t9848;
    let t10321 = F::new(0.212822999466489197e-4) * t9850;
    let t10322 = F::new(0.11974241701863808564e0) * t9861;
    let t10323 = F::new(0.40911992481368012596e-1) * t9865;
    let t10324 = F::new(0.5987120850931904282e-1) * t9870;
    let t10325 = F::new(0.1702583995731913576e-4) * t9933;
    (t10314, t10316, t10318, t10319, t10320, t10321, t10322, t10323, t10324, t10325)
}
