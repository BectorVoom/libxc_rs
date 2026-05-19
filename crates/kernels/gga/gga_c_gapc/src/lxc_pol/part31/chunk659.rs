//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 659/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk659<F: Float>(t3909: F, t576: F, t3763: F, t3776: F, t3793: F, t3835: F, t3836: F, t3838: F, t3839: F, t3840: F, t3842: F, t3843: F, t3844: F) -> (F, F) {
    let t3910 = t576 * t3909;
    let t3914 = t3835 - t3836 - F::cast_from(0.12650553385416666667e-5_f64) * t3763 + t3838 - t3839 - t3840 + F::cast_from(0.57970906942607043475e-5_f64) * t3776 - t3842 + t3843 + t3844 - F::cast_from(0.90579542097823505428e-7_f64) * t3793;
    (t3910, t3914)
}
