//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1089/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1089(t1562: f64, t8048: f64, t2474: f64, t934: f64, t289: f64, t36453: f64, t36464: f64, t37948: f64, t40715: f64, t40719: f64, t40732: f64, t40736: f64, t40740: f64, t40747: f64, t40757: f64, t40760: f64, t40762: f64, t40764: f64, t40772: f64, t40776: f64, t534: f64, t72: f64, t8291: f64) -> f64 {
    let t43722 = 0.4726e1_f64 * t1562 * t8048;
    let t43723 = t934 * t2474;
    let t43730 = -0.39726959900411316772e-4_f64 * t36453 + 0.3842256877732895568e-2_f64 * t40715 - 0.86737941314158990616e-4_f64 * t40719 + 0.3405167991463827152e-4_f64 * t40732 - 0.40911992481368012596e-1_f64 * t40736 + 0.5454932330849068346e-1_f64 * t40740 + 0.1702583995731913576e-4_f64 * t40747 + t72 * t534 * t8291 - 0.49658699875514145964e-4_f64 * t36464 + 0.8980681276397856423e-1_f64 * t40757 - 0.16364796992547205038e0_f64 * t40760 + t37948 - t43722 - 0.4726e1_f64 * t289 * t43723 + 0.5107751987195740728e-4_f64 * t40762 + 0.1702583995731913576e-4_f64 * t40764 + 0.212822999466489197e-4_f64 * t40772 + 0.5107751987195740728e-4_f64 * t40776;
    t43730
}
