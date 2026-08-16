//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta861 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2751;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta861(t1204: f64, t6695: f64, t1276: f64, t6573: f64, t12587: f64, t6748: f64, t21635: f64, t3801: f64, t3857: f64, t6801: f64, t3860: f64, t123: f64, t2630: f64, t6800: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t73222, t73236, t73252, t73273, t73321, t73329, t73341) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2751(t1204, t6695, t1276, t6573, t12587, t6748, t21635, t3801, t3857, t6801, t3860, t123, t2630, t6800);
    (t73222, t73236, t73252, t73273, t73321, t73329, t73341)
}
