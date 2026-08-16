//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta306 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1201;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta306(t1179: f64, t1188: f64, t12547: f64, t1196: f64, t300: f64, t3488: f64, t1198: f64, t3531: f64, t3539: f64, t3543: f64, t3535: f64, t12485: f64, t12487: f64, t3523: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12564, t12566, t12571, t12573, t12575, t12577, t12579, t12581) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1201(t1179, t1188, t12547, t1196, t300, t3488, t1198, t3531, t3539, t3543, t3535, t12485, t12487, t3523);
    (t12564, t12566, t12571, t12573, t12575, t12577, t12579, t12581)
}
