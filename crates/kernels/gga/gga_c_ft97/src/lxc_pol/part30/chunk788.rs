//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 788/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk788<F: Float>(t1506: F, t7022: F, t193: F, t1253: F, t7585: F, t1248: F, t7679: F, t2843: F, t7672: F, t10697: F, t25188: F, t7114: F, t1501: F, t7124: F, t6961: F, t7150: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36048 = t7022 * t1506;
    let t36049 = t193 * t36048;
    let t36056 = t7585 * t1253;
    let t36057 = t193 * t36056;
    let t36060 = t7679 * t1248;
    let t36061 = t2843 * t36060;
    let t36063 = t7672 * t1248;
    let t36064 = t10697 * t36063;
    let t36066 = t25188 * t7114;
    let t36068 = t1501 * t7124;
    let t36069 = t2843 * t36068;
    let t36071 = t6961 * t7150;
    (t36048, t36049, t36056, t36057, t36060, t36061, t36063, t36064, t36066, t36068, t36069, t36071)
}
