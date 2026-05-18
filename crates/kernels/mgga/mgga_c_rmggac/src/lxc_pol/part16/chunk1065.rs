//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1065/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1065<F: Float>(t42609: F, t42621: F, t44767: F, t44771: F, t44773: F, t44777: F, t44781: F, t44784: F, t44786: F, t44789: F, t44793: F, t44795: F, t44799: F, t44801: F, t44808: F, t44812: F, t44816: F, t44818: F) -> F {
    let t48193 = -F::new(0.1702583995731913576e-4) * t44767 + F::new(0.5107751987195740728e-4) * t44771 - F::new(0.1702583995731913576e-4) * t44773 - F::new(0.47885174879960069325e-4) * t44777 + t42609 - F::new(0.638468998399467591e-4) * t44781 - t42621 + F::new(0.11918087970123395032e-3) * t44784 + F::new(0.5454932330849068346e-1) * t44786 + F::new(0.2727466165424534173e-1) * t44789 + F::new(0.638468998399467591e-4) * t44793 + F::new(0.40911992481368012596e-1) * t44795 - F::new(0.5107751987195740728e-4) * t44799 + F::new(0.5107751987195740728e-4) * t44801 + F::new(0.5107751987195740728e-4) * t44808 - F::new(0.212822999466489197e-4) * t44812 - F::new(0.638468998399467591e-4) * t44816 - F::new(0.10909864661698136692e0) * t44818;
    t48193
}
