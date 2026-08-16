//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta885 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta885(t22352: f64, t2435: f64, t2782: f64, t4086: f64, t543: f64, t74965: f64, t4003: f64, t5744: f64, t74982: f64, t74700: f64, t4100: f64, t22394: f64, t686: f64, t72: f64, t9680: f64, t21969: f64, t566: f64, t1450: f64, t22461: f64, t116: f64, t21813: f64, t21830: f64, t625: f64, t2289: f64, t5916: f64, t21877: f64, t1507: f64, t2357: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t75274, t75298, t75302, t75307, t75336) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2798(t22352, t2435, t2782, t4086, t543, t74965, t4003, t5744, t74982, t74700, t4100, t22394, t686, t72, t9680);
        let (t75379, t75389, t75439, t75526, t75540, t75542, t75625) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2799(t21969, t566, t1450, t22461, t116, t21813, t21830, t625, t2289, t5916, t21877, t1507, t2357);
    (t75274, t75298, t75302, t75307, t75336, t75379, t75389, t75439, t75526, t75540, t75542, t75625)
}
