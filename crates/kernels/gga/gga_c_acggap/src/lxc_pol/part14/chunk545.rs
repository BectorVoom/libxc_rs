//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 545/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk545<F: Float>(t3937: F, t865: F, t191: F, t813: F, t4: F, t483: F, t657: F, t1357: F, t807: F, t2847: F, t1388: F, t224: F) -> (F, F, F, F, F, F) {
    let t3939 = F::new(0.39512695097613069591e1) * t3937 * t865;
    let t3952 = F::new(1.0) / t813 / t191;
    let t3992 = t483 * t4;
    let t3993 = t3992 * t657;
    let t4030 = t1357 * t807;
    let t4044 = F::new(32.0) * t2847;
    let t4045 = t224 * t1388;
    (t3939, t3952, t3993, t4030, t4044, t4045)
}
