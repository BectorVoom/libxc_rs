//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta831(t1063: f64, t11986: f64, t247: f64, t6096: f64, t20112: f64, t359: f64, t19572: f64, t3302: f64, t12046: f64, t1678: f64, t342: f64, t1086: f64, t6343: f64, t994: f64, t4772: f64, t4975: f64, t19462: f64, t3286: f64, t3298: f64, t6235: f64, t3316: f64, t19856: f64, t16543: f64, t4746: f64, t1647: f64, t16551: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67575, t67595, t67599, t67644, t67652) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691(t1063, t11986, t247, t6096, t20112, t359, t19572, t3302, t12046, t1678, t342, t1086, t6343, t994);
        let (t67668, t67714, t67725, t67790, t67825, t67927, t67969) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692(t4772, t4975, t19462, t3286, t3298, t6235, t3316, t1086, t19856, t16543, t4746, t1647, t16551);
    (t67575, t67595, t67599, t67644, t67652, t67668, t67714, t67725, t67790, t67825, t67927, t67969)
}
