//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta203 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk976;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk977;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta203<F: Float>(t1548: F, t775: F, t800: F, t4365: F, t837: F, t4364: F, t125: F, t1544: F, t2747: F, t1549: F, t2703: F, t124: F, t4343: F, t2749: F, t2488: F, t2653: F, t2666: F, t2678: F, t2691: F, t2695: F, t2702: F, t2716: F, t2730: F, t2739: F, t2745: F, t799: F, t4439: F) -> (F, F, F, F, F, F, F, F) {
        let (t4442, t4447, t4450, t4452, t4455, t4457) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk976::<F>(t1548, t775, t800, t4365, t837, t4364, t125, t1544, t2747, t1549, t2703, t124, t4343);
        let (t4458, t4462, t4468) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk977::<F>(t4457, t800, t2749, t4365, t2747, t2488, t2653, t2666, t2678, t2691, t2695, t2702, t2716, t2730, t2739, t2745, t4442, t4447, t4452, t4455, t799);
        let t4469 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk978::<F>(t4439, t4468);
    (t4442, t4447, t4450, t4452, t4457, t4458, t4462, t4469)
}
