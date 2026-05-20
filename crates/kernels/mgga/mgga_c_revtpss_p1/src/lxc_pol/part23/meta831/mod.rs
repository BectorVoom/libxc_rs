//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta831<F: Float>(t1063: F, t11986: F, t247: F, t6096: F, t20112: F, t359: F, t19572: F, t3302: F, t12046: F, t1678: F, t342: F, t1086: F, t6343: F, t994: F, t4772: F, t4975: F, t19462: F, t3286: F, t3298: F, t6235: F, t3316: F, t19856: F, t16543: F, t4746: F, t1647: F, t16551: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t67575, t67595, t67599, t67644, t67652) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2691::<F>(t1063, t11986, t247, t6096, t20112, t359, t19572, t3302, t12046, t1678, t342, t1086, t6343, t994);
        let (t67668, t67714, t67725, t67790, t67825, t67927, t67969) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2692::<F>(t4772, t4975, t19462, t3286, t3298, t6235, t3316, t1086, t19856, t16543, t4746, t1647, t16551);
    (t67575, t67595, t67599, t67644, t67652, t67668, t67714, t67725, t67790, t67825, t67927, t67969)
}
