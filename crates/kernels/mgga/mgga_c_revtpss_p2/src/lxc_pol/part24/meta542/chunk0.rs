//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1593/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1593<F: Float>(t45927: F, t45929: F, t45931: F, t45933: F, t45935: F, t45937: F, t45939: F, t45941: F, t45944: F, t45946: F, t45948: F, t45950: F, t45952: F) -> F {
    let t87072 = t45927 + t45929 + t45931 + t45933 + t45935 + t45937 + t45939 + t45941 + t45944 + t45946 + t45948 + t45950 + t45952;
    t87072
}
