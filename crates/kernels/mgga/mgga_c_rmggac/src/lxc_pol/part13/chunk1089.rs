//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1089/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1089<F: Float>(t1562: F, t8048: F, t2474: F, t934: F, t289: F, t36453: F, t36464: F, t37948: F, t40715: F, t40719: F, t40732: F, t40736: F, t40740: F, t40747: F, t40757: F, t40760: F, t40762: F, t40764: F, t40772: F, t40776: F, t534: F, t72: F, t8291: F) -> F {
    let t43722 = F::new(0.4726e1) * t1562 * t8048;
    let t43723 = t934 * t2474;
    let t43730 = -F::new(0.39726959900411316772e-4) * t36453 + F::new(0.3842256877732895568e-2) * t40715 - F::new(0.86737941314158990616e-4) * t40719 + F::new(0.3405167991463827152e-4) * t40732 - F::new(0.40911992481368012596e-1) * t40736 + F::new(0.5454932330849068346e-1) * t40740 + F::new(0.1702583995731913576e-4) * t40747 + t72 * t534 * t8291 - F::new(0.49658699875514145964e-4) * t36464 + F::new(0.8980681276397856423e-1) * t40757 - F::new(0.16364796992547205038e0) * t40760 + t37948 - t43722 - F::new(0.4726e1) * t289 * t43723 + F::new(0.5107751987195740728e-4) * t40762 + F::new(0.1702583995731913576e-4) * t40764 + F::new(0.212822999466489197e-4) * t40772 + F::new(0.5107751987195740728e-4) * t40776;
    t43730
}
