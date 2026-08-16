//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2677;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta687<F: Float>(t30: F, t21881: F, t508: F, t1518: F, t5517: F, t13584: F, t9375: F, t6785: F, t9335: F, t3833: F, t5824: F, t18280: F, t2255: F, t513: F, t5549: F, t605: F, zeta_threshold: F, t33: F, t6792: F, t9350: F, t3841: F, t6416: F, t1113: F, t20256: F, t516: F, t5557: F, t162: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t21882, t21891, t21901, t21905, t21906, t21911, t21917) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2677::<F>(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21918, t21923, t21931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2678::<F>(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, zeta_threshold);
    (t21882, t21891, t21901, t21905, t21906, t21911, t21918, t21923, t21931)
}
