//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1138/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1138<F: Float>(t34066: F, t34069: F, t34071: F, t34075: F, t34079: F, t34084: F, t34088: F, t34092: F, t34095: F, t34098: F, t34100: F, t11990: F, t19139: F, t2597: F) -> (F, F) {
    let t34102 = -F::new(0.33764099580923002116e-6) * t34066 - F::new(0.20010856351627032588e-7) * t34069 - F::new(0.20047434126173032506e-6) * t34071 - F::new(0.13097074855481695405e-8) * t34075 - F::new(0.16113527135189093757e-8) * t34079 + F::new(0.30361328125000000002e-3) * t34084 + F::new(0.4419710299937580002e-8) * t34088 - F::new(0.49190053374354708085e-8) * t34092 - F::new(0.33764099580923002116e-6) * t34095 + F::new(0.39291224566445086216e-8) * t34098 - F::new(0.18115908419564701086e-6) * t34100;
    let t34104 = t11990 * t2597 * t19139;
    (t34102, t34104)
}
