//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 572/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk572(t1421: f64, t456: f64, t4586: f64, t4587: f64, t4589: f64, t4591: f64, t4600: f64, t4606: f64, t4611: f64, t4616: f64, t4620: f64, t4626: f64, t4632: f64, t4654: f64, t4660: f64, t4686: f64, t4689: f64, t4794: f64, t604: f64) -> f64 {
    let t4797 = -t4586 + 0.8760572888888888889e-3_f64 * t4587 + 0.19711289e-2_f64 * t4589 - 0.13140859333333333333e-2_f64 * t4591 + 0.10950716111111111111e-2_f64 * t1421 * t4600 + 0.19711289e-2_f64 * t1421 * t4606 - 0.13140859333333333333e-2_f64 * t1421 * t4611 - 0.13140859333333333333e-2_f64 * t1421 * t4616 + 0.65704296666666666667e-3_f64 * t1421 * t4620 + 0.7391733375e-3_f64 * t456 * t4626 - 0.295669335e-2_f64 * t1421 * t4632 + 0.1478346675e-2_f64 * t456 * t4654 + 0.19711289e-2_f64 * t456 * t4660 - 0.98556445e-3_f64 * t456 * t4686 - 4.0_f64 * t4689 - 4.0_f64 * t604 * t4794;
    t4797
}
