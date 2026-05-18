//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 572/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk572<F: Float>(t1421: F, t456: F, t4586: F, t4587: F, t4589: F, t4591: F, t4600: F, t4606: F, t4611: F, t4616: F, t4620: F, t4626: F, t4632: F, t4654: F, t4660: F, t4686: F, t4689: F, t4794: F, t604: F) -> F {
    let t4797 = -t4586 + F::new(0.8760572888888888889e-3) * t4587 + F::new(0.19711289e-2) * t4589 - F::new(0.13140859333333333333e-2) * t4591 + F::new(0.10950716111111111111e-2) * t1421 * t4600 + F::new(0.19711289e-2) * t1421 * t4606 - F::new(0.13140859333333333333e-2) * t1421 * t4611 - F::new(0.13140859333333333333e-2) * t1421 * t4616 + F::new(0.65704296666666666667e-3) * t1421 * t4620 + F::new(0.7391733375e-3) * t456 * t4626 - F::new(0.295669335e-2) * t1421 * t4632 + F::new(0.1478346675e-2) * t456 * t4654 + F::new(0.19711289e-2) * t456 * t4660 - F::new(0.98556445e-3) * t456 * t4686 - F::new(4.0) * t4689 - F::new(4.0) * t604 * t4794;
    t4797
}
