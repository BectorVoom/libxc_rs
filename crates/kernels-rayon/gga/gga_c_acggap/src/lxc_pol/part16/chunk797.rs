//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 797/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk797(t2030: f64, t8920: f64, t2288: f64, t301: f64, t4262: f64, t1016: f64, t142: f64, t372: f64, t2060: f64, t7738: f64, t7740: f64, t7743: f64, t7748: f64, t7754: f64, t7756: f64, t7758: f64, t7760: f64, t8904: f64, t8909: f64, t8913: f64, t8917: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8921 = t2030 * t8920;
    let t8923 = t2288 * t301;
    let t8924 = t4262 * t8923;
    let t8925 = t2030 * t8924;
    let t8927 = t142 * t1016;
    let t8928 = t2288 * t372;
    let t8929 = t8927 * t8928;
    let t8930 = t2060 * t8929;
    let t8936 = 0.53592522647587171215e-3_f64 * t8904 + 0.21437009059034868486e-3_f64 * t8909 - 0.10718504529517434243e-3_f64 * t8913 - 0.4584375e-1_f64 * t8917 - 0.22921875e-1_f64 * t8921 - 0.22921875e-1_f64 * t8925 - 0.22921875e-1_f64 * t8930 - t7738 - t7740 + t7743 + t7748 - 0.14291339372689912324e-3_f64 * t7754 + 0.32155513588552302729e-2_f64 * t7756 + 0.12862205435420921092e-2_f64 * t7758 - 0.53592522647587171215e-3_f64 * t7760;
    (t8923, t8924, t8927, t8928, t8929, t8936)
}
