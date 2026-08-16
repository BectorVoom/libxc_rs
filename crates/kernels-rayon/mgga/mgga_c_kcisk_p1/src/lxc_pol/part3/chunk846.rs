//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 846/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk846(t12911: f64, t3679: f64, t12910: f64, t1175: f64, t3587: f64, t3598: f64, t3651: f64, t12831: f64, t4271: f64, t12: f64) -> (f64, f64, f64, f64) {
    let t12912 = t12911 * t3679;
    let t12914 = 0.96490945932906628932e2_f64 * t12910 * t12912;
    let t12916 = t3598 * t1175 * t3587;
    let t12919 = t3651 * t1175 * t3587;
    let t12921 = t4271 * t12831;
    let t12922 = t12 * t12921;
    (t12914, t12916, t12919, t12922)
}
