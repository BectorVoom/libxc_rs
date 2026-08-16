//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta351 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1321;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1322;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1323;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta351(t13191: f64, t2701: f64, t820: f64, t1484: f64, t2553: f64, t2563: f64, t4159: f64, t119: f64, t12971: f64, t210: f64, t4155: f64, t9573: f64, t2645: f64, t2684: f64, t4248: f64, t13076: f64, t13080: f64, t13084: f64, t13087: f64, t13173: f64, t13177: f64, t13182: f64, t13186: f64, t13190: f64, t2623: f64, t2643: f64, t2681: f64, t4167: f64, t4178: f64, t4257: f64, t787: f64, t817: f64, t831: f64, t843: f64, t9602: f64, t9604: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t13193, t13196) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1321(t13191, t2701, t820, t1484, t2553);
        let (t13198, t13202, t13204, t13208, t13210) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1322(t13196, t2701, t820, t2563, t4159, t119, t12971, t210, t4155, t9573, t2645, t2684, t4248);
        let t13213 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1323(t13076, t13080, t13084, t13087, t13173, t13177, t13182, t13186, t13190, t13193, t13198, t13202, t13204, t13208, t13210, t2623, t2643, t2681, t4167, t4178, t4257, t787, t817, t831, t843, t9602, t9604);
    (t13193, t13196, t13198, t13204, t13210, t13213)
}
