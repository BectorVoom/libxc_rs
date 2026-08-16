//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1537;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1538;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta428(t1079: f64, t1651: f64, t5015: f64, t4772: f64, t996: f64, t16313: f64, t4940: f64, t6258: f64, t999: f64, t1096: f64, t6244: f64, t6350: f64, t11121: f64, t3268: f64, t4946: f64, t1076: f64, t11224: f64, t16284: f64, t16312: f64, t16333: f64, t16371: f64, t16603: f64, t1696: f64, t3047: f64, t3058: f64, t3063: f64, t4747: f64, t4758: f64, t4935: f64, t4941: f64, t5016: f64, t6245: f64, t6251: f64, t995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19396, t19399, t19400, t19403, t19414, t19415, t19421, t19424) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1537(t1079, t1651, t5015, t4772, t996, t16313, t4940, t6258, t999, t1096, t6244, t6350);
        let (t19425, t19429, t19434) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1538(t11121, t19424, t1651, t3268, t4946, t1076, t11224, t16284, t16312, t16333, t16371, t16603, t1696, t19396, t19400, t19403, t19415, t19421, t3047, t3058, t3063, t4747, t4758, t4935, t4941, t5016, t6245, t6251, t995);
    (t19396, t19399, t19400, t19403, t19414, t19415, t19421, t19425, t19429, t19434)
}
