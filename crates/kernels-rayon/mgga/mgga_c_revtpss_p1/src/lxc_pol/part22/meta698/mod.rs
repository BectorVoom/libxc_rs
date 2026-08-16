//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta698 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2704;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2705;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2706;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta698(t22190: f64, t22203: f64, t22210: f64, t22220: f64, t225: f64, t1877: f64, t73: f64, t4010: f64, t6836: f64, t1353: f64, t5591: f64, t5651: f64, t1412: f64, t6816: f64, t1394: f64, t21969: f64, t1392: f64, t1395: f64, t1879: f64, t539: f64, t541: f64, t5644: f64, t5650: f64, t5652: f64, t5655: f64, t6832: f64, t6837: f64, t6840: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22223, t22229, t22236, t22237, t22240) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2704(t22190, t22203, t22210, t22220, t225, t1877, t73, t4010, t6836, t1353, t5591, t5651);
        let (t22245, t22246, t22249, t22252) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2705(t1412, t6816, t1353, t1394, t21969, t1392, t1395, t1877, t1879, t22223, t22229, t22237, t22240, t539, t541, t5644, t5650, t5652, t5655, t6832, t6837, t6840);
        let t22253 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2706(t22252, t543);
    (t22223, t22229, t22236, t22237, t22240, t22245, t22246, t22249, t22252, t22253)
}
