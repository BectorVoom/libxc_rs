//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1015/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1015(t1802: f64, t473: f64, t369: f64, t8923: f64, t1774: f64, t1828: f64, t2150: f64, t33414: f64, t33436: f64, t33441: f64, t33446: f64, t33449: f64, t33477: f64, t33484: f64, t33524: f64, t34945: f64, t34949: f64, t34952: f64, t34957: f64, t34961: f64, t34966: f64, t34969: f64, t34972: f64, t34982: f64, t8217: f64, t8926: f64, t8928: f64, t8932: f64, t8941: f64, t8946: f64, t8948: f64) -> (f64, f64, f64, f64) {
    let t34989 = t473 * t1802;
    let t34990 = t34989 * t369;
    let t34991 = t8923 * t34990;
    let t34994 = 0.99166211224136028853e-3_f64 * t8946 * t34945 + 0.11423947533020470523e1_f64 * t33484 * t34949 + 0.12395776403017003607e-3_f64 * t33524 * t34952 + 0.28234466758480466999e-3_f64 * t33414 * t34957 - 0.17135921299530705785e1_f64 * t33477 * t34961 - 0.28234466758480466999e-3_f64 * t8926 * t34966 + 0.57119737665102352616e0_f64 * t34969 * t8941 - 0.1859366460452550541e-3_f64 * t34972 * t8948 - 0.17347256376410398924e1_f64 * t33446 * t8217 + 0.17347256376410398924e1_f64 * t8932 * t33449 * t473 * t1828 - 0.17347256376410398924e1_f64 * t33436 * t34982 + 0.17347256376410398924e1_f64 * t33441 * t2150 * t473 * t1774 - 0.15058382271189582399e-2_f64 * t34991 * t8928;
    (t34989, t34990, t34991, t34994)
}
