//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2499;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta606<F: Float>(t19399: F, t996: F, t16313: F, t4940: F, t6258: F, t999: F, t1079: F, t1096: F, t6244: F, t6350: F, t11121: F, t1651: F, t3268: F, t4946: F, t1076: F, t11224: F, t16284: F, t16312: F, t16333: F, t16371: F, t16603: F, t1696: F, t19396: F, t3047: F, t3058: F, t3063: F, t4747: F, t4758: F, t4935: F, t4941: F, t5016: F, t6245: F, t6251: F, t995: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19400, t19403, t19414) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2499::<F>(t19399, t996, t16313, t4940, t6258, t999);
        let (t19415, t19421, t19425, t19428, t19429, t19434) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2500::<F>(t19414, t996, t1079, t1096, t6244, t6350, t11121, t1651, t3268, t4946, t1076, t11224, t16284, t16312, t16333, t16371, t16603, t1696, t19396, t19400, t19403, t3047, t3058, t3063, t4747, t4758, t4935, t4941, t5016, t6245, t6251, t995);
    (t19400, t19403, t19414, t19415, t19421, t19425, t19428, t19429, t19434)
}
