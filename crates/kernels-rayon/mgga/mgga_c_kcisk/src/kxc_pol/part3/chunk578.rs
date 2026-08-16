//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 578/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk578(t4856: f64, t608: f64, t1724: f64, t1725: f64, t606: f64, t609: f64, t1709: f64, t4834: f64, t4838: f64, t4842: f64, t4845: f64, t4848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4857 = 1.0_f64 / t4856;
    let t4858 = t608 * t4857;
    let t4859 = t1724 * t1724;
    let t4860 = t4859 * t1725;
    let t4864 = 1.0_f64 / t609 / t606;
    let t4865 = t1709 * t1709;
    let t4866 = t4864 * t4865;
    let t4868 = 4.0_f64 / 9.0_f64 * t4834;
    let t4873 = t4868 + 2.0_f64 / 9.0_f64 * t4838 - 2.0_f64 / 9.0_f64 * t4842 + 2.0_f64 / 3.0_f64 * t4845 - t4848 / 3.0_f64;
    (t4857, t4858, t4859, t4860, t4864, t4865, t4866, t4873)
}
