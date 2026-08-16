//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2650/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2650(t39365: f64, t56168: f64, t54380: f64, t54382: f64, t39374: f64, t54389: f64, t56185: f64, t54392: f64, t15883: f64, t19577: f64, t19596: f64, t19631: f64, t3918: f64, t39400: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t5126: f64, t5127: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74040 = 0.56968947174242584612e-3_f64 * t39365;
    let t74041 = 0.35089341735807877242e1_f64 * t56168;
    let t74042 = 0.48796115851357829289e-1_f64 * t54380;
    let t74043 = 0.14447919941302971323e1_f64 * t54382;
    let t74044 = 0.10254018858216406658e4_f64 * t39374;
    let t74046 = 0.17544670867903938621e1_f64 * t54389;
    let t74056 = 24.0_f64 * t56185;
    let t74057 = 0.10526802520742363173e2_f64 * t54392;
    let t74058 = 18.0_f64 * t15883 * t5126 * t6347 - 9.0_f64 * t19577 * t19596 * t3918 + 18.0_f64 * t19631 * t5126 * t5127 - t39400 + t39408 + t39411 + t39463 - t39468 - t74046 - t74056 + t74057;
    (t74040, t74041, t74042, t74043, t74044, t74046, t74056, t74057, t74058)
}
