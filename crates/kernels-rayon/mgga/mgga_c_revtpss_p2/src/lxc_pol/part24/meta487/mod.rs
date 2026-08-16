//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1480;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta487(t12587: f64, t6748: f64, t3857: f64, t6801: f64, t3860: f64, t3863: f64, t123: f64, t2630: f64, t6800: f64, t2608: f64, t512: f64, t1317: f64, t22195: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t73252, t73321, t73329, t73331, t73341, t73350, t73360) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1480(t12587, t6748, t3857, t6801, t3860, t3863, t123, t2630, t6800, t2608, t512, t1317, t22195);
    (t73252, t73321, t73329, t73331, t73341, t73350, t73360)
}
