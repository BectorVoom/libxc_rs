//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 659/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk659<F: Float>(t10315: F, t1923: F, t2265: F, t9846: F, t9848: F, t9850: F, t9861: F, t9865: F, t9870: F, t9933: F, t9936: F, t9939: F, t530: F, t9639: F, t9949: F, t9952: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10316 = 0.11974241701863808564e0 * t10315;
    let t10317 = t1923 * t2265;
    let t10318 = 0.2363e1 * t10317;
    let t10319 = 0.212822999466489197e-4 * t9846;
    let t10320 = 0.1702583995731913576e-4 * t9848;
    let t10321 = 0.212822999466489197e-4 * t9850;
    let t10322 = 0.11974241701863808564e0 * t9861;
    let t10323 = 0.40911992481368012596e-1 * t9865;
    let t10324 = 0.5987120850931904282e-1 * t9870;
    let t10325 = 0.1702583995731913576e-4 * t9933;
    let t10329 = 0.5107751987195740728e-4 * t9936;
    let t10330 = 0.10215503974391481456e-3 * t9939;
    let t10332 = t530 * t9639;
    let t10333 = 0.4726e1 * t10332;
    let t10334 = 0.35922725105591425692e0 * t9949;
    let t10335 = 0.11974241701863808564e0 * t9952;
    (t10316, t10318, t10319, t10320, t10321, t10322, t10323, t10324, t10325, t10329, t10330, t10333, t10334, t10335)
}
