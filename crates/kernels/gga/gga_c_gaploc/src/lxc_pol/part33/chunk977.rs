//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 977/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk977<F: Float>(t10820: F, t10915: F, t10914: F, t3473: F, t549: F, t2033: F, t3040: F, t9823: F, t1022: F, t2536: F, t2021: F, t2009: F) -> (F, F, F, F, F, F, F, F) {
    let t10916 = t10915 * t10820;
    let t10918 = F::new(0.21450293971110256001e1) * t10914 * t10916;
    let t10919 = t549 * t3473;
    let t10920 = t2033 * t10919;
    let t10921 = F::new(0.29792074959875355558e-1) * t10920;
    let t10923 = F::new(0.35750489951850426669e0) * t9823 * t3040;
    let t10924 = t2536 * t1022;
    let t10925 = t2021 * t10924;
    let t10927 = F::new(0.35750489951850426669e0) * t10925 * t2009;
    (t10916, t10918, t10919, t10921, t10923, t10924, t10925, t10927)
}
