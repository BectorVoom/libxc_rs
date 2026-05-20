//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta415 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1797;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta415<F: Float>(t18426: F, t4364: F, t4366: F, t2741: F, t5980: F, t4365: F, t4424: F, t837: F, t125: F, t5966: F, t10770: F, t2652: F, t5993: F) -> (F, F, F, F, F, F, F) {
        let (t18456, t18459, t18462, t18466, t18469, t18471, t18475) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1797::<F>(t18426, t4364, t4366, t2741, t5980, t4365, t4424, t837, t125, t5966, t10770, t2652, t5993);
    (t18456, t18459, t18462, t18466, t18469, t18471, t18475)
}
