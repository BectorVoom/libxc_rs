//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 615/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk615<F: Float>(t7442: F, t7445: F, t7451: F, t7458: F, t7464: F, t7470: F, t7479: F, t7485: F, t7495: F, t7499: F, t7502: F, t7506: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8096 = F::new(0.5987120850931904282e-1) * t7442;
    let t8097 = F::new(0.8980681276397856423e-1) * t7445;
    let t8098 = F::new(0.1702583995731913576e-4) * t7451;
    let t8099 = F::new(0.212822999466489197e-4) * t7458;
    let t8100 = F::new(0.1702583995731913576e-4) * t7464;
    let t8101 = F::new(0.5107751987195740728e-4) * t7470;
    let t8102 = F::new(0.1702583995731913576e-4) * t7479;
    let t8103 = F::new(0.5107751987195740728e-4) * t7485;
    let t8109 = F::new(0.40911992481368012596e-1) * t7495;
    let t8110 = F::new(0.20455996240684006298e-1) * t7499;
    let t8111 = F::new(0.5454932330849068346e-1) * t7502;
    let t8112 = F::new(0.2727466165424534173e-1) * t7506;
    (t8096, t8097, t8098, t8099, t8100, t8101, t8102, t8103, t8109, t8110, t8111, t8112)
}
