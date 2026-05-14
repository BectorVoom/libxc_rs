//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1075/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1075<F: Float>(t13756: F, t13763: F, t14390: F, t14821: F, t2182: F, t3944: F, t3946: F, t4062: F, t4063: F, t4123: F, t50818: F, t52836: F, t52837: F, t52841: F, t52847: F, t52853: F, t52855: F, t52860: F, t52861: F, t52870: F, t8574: F, t944: F, t9740: F) -> (F,) {
    let t52874 = 6.0 * t13756 * t14390 * t2182 - 6.0 * t13763 * t14821 * t3946 + 3.0 * t3944 * t3946 * t8574 - 3.0 * t3946 * t4063 * t52837 - 3.0 * t3946 * t4063 * t52847 - t4062 * t4063 * t9740 - 6.0 * t4062 * t50818 * t52870 - 2.0 * t4062 * t52861 * t944 + 6.0 * t4123 * t52841 + t52836 + t52853 + t52855 - t52860;
    (t52874,)
}
