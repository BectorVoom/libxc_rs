//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 823/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk823(t7738: f64, t7740: f64, t7743: f64, t7748: f64, t7754: f64, t7756: f64, t7758: f64, t7760: f64, t8904: f64, t8909: f64, t8913: f64, t8917: f64, t8921: f64, t8925: f64, t8930: f64) -> f64 {
    let t8936 = 0.53592522647587171215e-3_f64 * t8904 + 0.21437009059034868486e-3_f64 * t8909 - 0.10718504529517434243e-3_f64 * t8913 - 0.4584375e-1_f64 * t8917 - 0.22921875e-1_f64 * t8921 - 0.22921875e-1_f64 * t8925 - 0.22921875e-1_f64 * t8930 - t7738 - t7740 + t7743 + t7748 - 0.14291339372689912324e-3_f64 * t7754 + 0.32155513588552302729e-2_f64 * t7756 + 0.12862205435420921092e-2_f64 * t7758 - 0.53592522647587171215e-3_f64 * t7760;
    t8936
}
