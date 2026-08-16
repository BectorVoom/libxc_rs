//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta660(t6897: f64, t6907: f64, t90544: f64, t26203: f64, t6883: f64, t7700: f64, t80645: f64, t225: f64, t26219: f64, t214: f64, t5318: f64, t26378: f64, t6914: f64, t1372: f64, t1799: f64, t26411: f64, t22704: f64, t22705: f64, t5345: f64, t22690: f64, t552: f64, t26447: f64, t90607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90702, t90708, t90724, t90732, t90739, t90749) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081(t6897, t6907, t90544, t26203, t6883, t7700, t80645, t225, t26219, t214, t5318, t26378, t6914);
        let (t90750, t90754, t90760, t90782, t90787, t90789) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2082(t90749, t1372, t1799, t26411, t6914, t22704, t22705, t5345, t22690, t552, t26447, t90607);
    (t90702, t90708, t90724, t90732, t90739, t90750, t90754, t90760, t90782, t90787, t90789)
}
