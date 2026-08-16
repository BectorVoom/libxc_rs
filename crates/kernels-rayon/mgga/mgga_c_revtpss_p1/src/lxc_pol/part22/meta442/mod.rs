//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2084;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2085;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta442(t1568: f64, t2718: f64, t4469: f64, t822: f64, t10923: f64, t10925: f64, t10930: f64, t10935: f64, t10939: f64, t10948: f64, t10961: f64, t10964: f64, t10966: f64, t10969: f64, t10971: f64, t10974: f64, t14507: f64, t2646: f64, t2724: f64, t4514: f64, t4526: f64, t820: f64, t837: f64, t14540: f64, t14572: f64, t14953: f64, t868: f64, t4533: f64, t72: f64, t686: f64, t2465: f64, t1569: f64, t867: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14961, t14972, t14976) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2084(t1568, t2718, t4469, t822, t10923, t10925, t10930, t10935, t10939, t10948, t10961, t10964, t10966, t10969, t10971, t10974, t14507, t2646, t2724, t4514, t4526, t820, t837);
        let (t14978, t14979, t14982, t14983, t14985, t14986, t14987) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2085(t14540, t14572, t14953, t14976, t868, t4533, t72, t686, t2465, t1569, t867, t786);
    (t14961, t14972, t14978, t14979, t14982, t14983, t14985, t14986, t14987)
}
