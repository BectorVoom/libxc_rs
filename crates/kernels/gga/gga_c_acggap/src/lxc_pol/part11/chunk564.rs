//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 564/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk564<F: Float>(t323: F, t3930: F, t868: F, t880: F, t180: F, t3054: F, t865: F, t191: F, t813: F, t301: F, t467: F, t4: F, t483: F) -> (F, F, F, F, F, F) {
    let t3932 = F::new(0.19756347548806534796e1) * t3930 * t323;
    let t3935 = t868 * t880;
    let t3937 = t3054 * t180;
    let t3939 = F::new(0.39512695097613069591e1) * t3937 * t865;
    let t3952 = F::new(1.0) / t813 / t191;
    let t3984 = t467 * t301;
    let t3992 = t483 * t4;
    (t3932, t3935, t3939, t3952, t3984, t3992)
}
