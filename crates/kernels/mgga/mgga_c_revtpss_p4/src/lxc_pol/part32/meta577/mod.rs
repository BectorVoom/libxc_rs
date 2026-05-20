//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1903;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta577<F: Float>(t2453: F, t3908: F, t8086: F, t28829: F, t689: F, t25899: F, t26271: F, t27884: F, t28862: F, t686: F, t72: F, t25895: F, t102218: F, t25878: F, t2470: F, t28844: F, t7284: F, t26292: F, t1904: F, t26354: F, t27899: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t102266, t102268, t102270, t102272, t102274, t102276) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1903::<F>(t2453, t3908, t8086, t28829, t689, t25899, t26271, t27884, t28862, t686, t72, t25895);
        let (t102293, t102295, t102296, t102298, t102306, t102309) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1904::<F>(t102218, t25878, t2470, t28844, t7284, t26292, t27884, t1904, t26354, t689, t26271, t27899);
    (t102266, t102268, t102270, t102272, t102274, t102276, t102293, t102295, t102296, t102298, t102306, t102309)
}
