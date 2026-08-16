//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1062;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1063;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta284(t19462: f64, t225: f64, t3011: f64, t6205: f64, t3153: f64, t6305: f64, t1647: f64, t4980: f64, t359: f64, t6343: f64, t1086: f64, t6235: f64, t4995: f64, t6299: f64, t1678: f64, t3298: f64, t342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t19463 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1062(t19462, t225);
        let (t19467, t19501) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1063(t3011, t6205, t3153, t6305);
        let (t19526, t19556, t19566, t19569, t19572, t19602, t19603) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1064(t1647, t4980, t359, t6343, t1086, t6235, t4995, t3153, t6299, t1678, t3298, t342);
    (t19463, t19467, t19501, t19526, t19556, t19566, t19569, t19572, t19602, t19603)
}
