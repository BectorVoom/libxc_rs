//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 986/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk986<F: Float>(t139212: F, t139352: F, t148284: F, t34822: F, t379: F, t23667: F, t5899: F, t32897: F, t32899: F, t3450: F, t36571: F, t637: F, t139214: F, t139224: F, t26909: F, t3052: F, t32898: F, t3628: F, t5889: F) -> (F, F, F, F, F, F) {
    let t148286 = t139212 * t139352 * t148284;
    let t148288 = t34822 * t379;
    let t148290 = t5899 * t23667 * t148288;
    let t148295 = t32897 * t637 * t36571 * t32899 * t3450;
    let t148299 = t32897 * t139224 * t139214 * t26909;
    let t148304 = t5889 * t3628 * t32898 * t32899 * t3052;
    (t148286, t148288, t148290, t148295, t148299, t148304)
}
