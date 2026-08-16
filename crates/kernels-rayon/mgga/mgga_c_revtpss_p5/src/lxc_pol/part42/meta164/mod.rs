//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta164 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk709;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk710;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk711;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta164(t4457: f64, t800: f64, t2749: f64, t4365: f64, t2747: f64, t2488: f64, t2653: f64, t2666: f64, t2678: f64, t2691: f64, t2695: f64, t2702: f64, t2716: f64, t2730: f64, t2739: f64, t2745: f64, t4442: f64, t4447: f64, t4452: f64, t4455: f64, t799: f64, t4439: f64, t225: f64, t1568: f64, t213: f64, t1580: f64, t779: f64, t689: f64, t1579: f64, t72: f64, t686: f64, t2465: f64, t886: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4458, t4462, t4468) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk709(t4457, t800, t2749, t4365, t2747, t2488, t2653, t2666, t2678, t2691, t2695, t2702, t2716, t2730, t2739, t2745, t4442, t4447, t4452, t4455, t799);
        let t4469 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk710(t4439, t4468);
        let (t4470, t4474, t4477, t4478, t4480, t4481, t4482, t4486) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk711(t225, t4469, t1568, t213, t1580, t779, t689, t1579, t72, t686, t2465, t886);
    (t4458, t4462, t4469, t4470, t4474, t4477, t4478, t4480, t4481, t4482, t4486)
}
