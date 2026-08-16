//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk706;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk707;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk708;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta164<F: Float>(t4457: F, t800: F, t2749: F, t4365: F, t2747: F, t2488: F, t2653: F, t2666: F, t2678: F, t2691: F, t2695: F, t2702: F, t2716: F, t2730: F, t2739: F, t2745: F, t4442: F, t4447: F, t4452: F, t4455: F, t799: F, t4439: F, t225: F, t1568: F, t213: F, t1580: F, t779: F, t689: F, t1579: F, t72: F, t686: F, t2465: F, t886: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4458, t4462, t4468) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk706::<F>(t4457, t800, t2749, t4365, t2747, t2488, t2653, t2666, t2678, t2691, t2695, t2702, t2716, t2730, t2739, t2745, t4442, t4447, t4452, t4455, t799);
        let t4469 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk707::<F>(t4439, t4468);
        let (t4470, t4474, t4477, t4478, t4480, t4481, t4482, t4486) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk708::<F>(t225, t4469, t1568, t213, t1580, t779, t689, t1579, t72, t686, t2465, t886);
    (t4458, t4462, t4469, t4470, t4474, t4477, t4478, t4480, t4481, t4482, t4486)
}
