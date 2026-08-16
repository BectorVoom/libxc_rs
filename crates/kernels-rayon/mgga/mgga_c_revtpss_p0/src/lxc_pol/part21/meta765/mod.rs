//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta765 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta765(t39774: f64, t15071: f64, t892: f64, t14330: f64, t14389: f64, t2251: f64, t14322: f64, t2516: f64, t39779: f64, t2496: f64, t14426: f64, t177: f64, t762: f64, t10600: f64, t18259: f64, t14325: f64, t14390: f64, t14468: f64, t1544: f64, t2403: f64, t2404: f64, t39783: f64, t41197: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49945, t49950, t49956, t49958, t49959, t49964, t49966) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714(t39774, t15071, t892, t14330, t14389, t2251, t14322, t2516, t39779, t2496, t14426, t177, t762);
        let (t49967, t49969, t49971, t49972) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2715(t49966, t10600, t18259, t14325, t14390, t14468, t1544, t2403, t2404, t39783, t41197, t49950, t49956, t49958, t49959, t49964, t775);
    (t49945, t49956, t49958, t49959, t49964, t49967, t49969, t49971, t49972)
}
