//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 785/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk785<F: Float>(t28: F, t34939: F, t89: F, t34918: F, t526: F, t27: F, t32945: F, t32961: F, t32978: F, t34851: F, t34856: F, t34921: F, t34925: F, t34929: F, t34933: F, t34937: F) -> (F, F, F, F) {
    let t34940 = t28 * t34939;
    let t34941 = t89 * t34940;
    let t34943 = t526 * t34918;
    let t34945 = t89 * t27 * t34943;
    let t34946 = t32945 + t34851 / 6.0 + t34856 - t34921 / 2.0 - t32961 - 2.0 / 3.0 * t34925 - 6.0 * t34929 + 4.0 * t34933 + t32978 + t34937 / 3.0 + 2.0 * t34941 - t34945;
    (t34941, t34943, t34945, t34946)
}
