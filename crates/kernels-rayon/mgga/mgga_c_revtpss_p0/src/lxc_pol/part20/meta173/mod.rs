//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta173 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk912;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta173(t9484: f64, t9543: f64, t520: f64, t512: f64, t1450: f64, t4135: f64, t177: f64, t3850: f64, t762: f64, t749: f64, t1353: f64, t198: f64, t4139: f64, t566: f64, t9399: f64, t9400: f64, t9405: f64, t9407: f64, t9409: f64, t9412: f64, t9415: f64, t9421: f64, t9423: f64, t9427: f64, t9430: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9544, t9545, t9546, t9547, t9551, t9553, t9554, t9556, t9557) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk912(t9484, t9543, t520, t512, t1450, t4135, t177, t3850, t762, t749, t1353, t198, t4139, t566, t9399, t9400, t9405, t9407, t9409, t9412, t9415, t9421, t9423, t9427, t9430);
    (t9544, t9545, t9546, t9547, t9551, t9553, t9554, t9556, t9557)
}
