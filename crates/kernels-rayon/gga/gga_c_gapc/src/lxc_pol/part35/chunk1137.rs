//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1137/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1137(t34066: f64, t34069: f64, t34071: f64, t34075: f64, t34079: f64, t34084: f64, t34088: f64, t34092: f64, t34095: f64, t34098: f64, t34100: f64, t11990: f64, t19139: f64, t2597: f64) -> (f64, f64) {
    let t34102 = -0.33764099580923002116e-6_f64 * t34066 - 0.20010856351627032588e-7_f64 * t34069 - 0.20047434126173032506e-6_f64 * t34071 - 0.13097074855481695405e-8_f64 * t34075 - 0.16113527135189093757e-8_f64 * t34079 + 0.30361328125000000002e-3_f64 * t34084 + 0.4419710299937580002e-8_f64 * t34088 - 0.49190053374354708085e-8_f64 * t34092 - 0.33764099580923002116e-6_f64 * t34095 + 0.39291224566445086216e-8_f64 * t34098 - 0.18115908419564701086e-6_f64 * t34100;
    let t34104 = t11990 * t2597 * t19139;
    (t34102, t34104)
}
