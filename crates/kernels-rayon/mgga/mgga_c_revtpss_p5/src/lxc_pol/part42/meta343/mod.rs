//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta343 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1147;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta343(t1062: f64, t11940: f64, t3111: f64, t4834: f64, t11262: f64, t1670: f64, t1041: f64, t3172: f64, t4824: f64, t3127: f64, t3211: f64, t4845: f64, t1053: f64, t4857: f64, t1663: f64, t371: f64, t676: f64, t1025: f64, t11922: f64, t4901: f64, t4899: f64, t4874: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15716, t15724, t15732, t15736, t15744) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1147(t1062, t11940, t3111, t4834, t11262, t1670, t1041, t3172, t4824, t3127, t3211, t4845);
        let (t15745, t15750, t15754, t15771) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1148(t1053, t4857, t1663, t371, t676, t1025, t11922, t4901, t4899, t3172, t4874, t3127);
    (t15716, t15724, t15732, t15736, t15744, t15745, t15750, t15754, t15771)
}
