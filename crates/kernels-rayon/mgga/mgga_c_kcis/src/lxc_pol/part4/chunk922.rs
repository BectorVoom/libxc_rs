//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 922/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk922(t2675: f64, t2683: f64, t2366: f64, t2375: f64, t678: f64, t859: f64, t47: f64, t8680: f64, t8656: f64, t680: f64, t8698: f64, t194: f64, t2679: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8798 = t2675 * t2683;
    let t8808 = t2366 * t2375;
    let t8809 = t8808 * t678;
    let t8812 = t859 * t2366;
    let t8815 = t47 * t8680;
    let t8816 = t8656 * t2375;
    let t8819 = t8698 * t680;
    let t8823 = 1.0_f64 / t2679 / t194;
    (t8798, t8809, t8812, t8815, t8816, t8819, t8823)
}
