//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 572/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk572<F: Float>(t1421: F, t456: F, t4586: F, t4587: F, t4589: F, t4591: F, t4600: F, t4606: F, t4611: F, t4616: F, t4620: F, t4626: F, t4632: F, t4654: F, t4660: F, t4686: F, t4689: F, t4794: F, t604: F) -> F {
    let t4797 = -t4586 + F::cast_from(0.8760572888888888889e-3_f64) * t4587 + F::cast_from(0.19711289e-2_f64) * t4589 - F::cast_from(0.13140859333333333333e-2_f64) * t4591 + F::cast_from(0.10950716111111111111e-2_f64) * t1421 * t4600 + F::cast_from(0.19711289e-2_f64) * t1421 * t4606 - F::cast_from(0.13140859333333333333e-2_f64) * t1421 * t4611 - F::cast_from(0.13140859333333333333e-2_f64) * t1421 * t4616 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t4620 + F::cast_from(0.7391733375e-3_f64) * t456 * t4626 - F::cast_from(0.295669335e-2_f64) * t1421 * t4632 + F::cast_from(0.1478346675e-2_f64) * t456 * t4654 + F::cast_from(0.19711289e-2_f64) * t456 * t4660 - F::cast_from(0.98556445e-3_f64) * t456 * t4686 - F::cast_from(4.0_f64) * t4689 - F::cast_from(4.0_f64) * t604 * t4794;
    t4797
}
