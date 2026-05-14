//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1054/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1054<F: Float>(t35031: F, t35034: F, t35036: F, t35039: F, t35041: F, t35045: F, t35048: F, t35051: F, t35056: F, t35059: F, t35062: F, t1666: F, t20461: F, t27867: F, t2993: F, t519: F, t6: F) -> (F, F) {
    let t35064 = -0.20241536458333333334e-4 * t35031 - 0.2209926229259557733e-7 * t35034 - 0.25340269868817520618e-3 * t35036 - 0.12650960286458333334e-5 * t35039 - 0.28985453471303521737e-5 * t35041 - 0.19336854506021130164e-8 * t35045 - 0.40483072916666666668e-4 * t35048 - 0.49240895655712845849e-7 * t35051 + 0.78584976712469872988e-8 * t35056 + 0.21103240995305505364e-7 * t35059 - 0.49522272202316919254e-5 * t35062;
    let t35069 = t2993 * t519 * t20461 * t1666 * t6 * t27867;
    (t35064, t35069)
}
