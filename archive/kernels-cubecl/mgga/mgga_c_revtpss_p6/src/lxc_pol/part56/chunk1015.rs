//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1015/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1015<F: Float>(t1802: F, t473: F, t369: F, t8923: F, t1774: F, t1828: F, t2150: F, t33414: F, t33436: F, t33441: F, t33446: F, t33449: F, t33477: F, t33484: F, t33524: F, t34945: F, t34949: F, t34952: F, t34957: F, t34961: F, t34966: F, t34969: F, t34972: F, t34982: F, t8217: F, t8926: F, t8928: F, t8932: F, t8941: F, t8946: F, t8948: F) -> (F, F, F, F) {
    let t34989 = t473 * t1802;
    let t34990 = t34989 * t369;
    let t34991 = t8923 * t34990;
    let t34994 = F::cast_from(0.99166211224136028853e-3_f64) * t8946 * t34945 + F::cast_from(0.11423947533020470523e1_f64) * t33484 * t34949 + F::cast_from(0.12395776403017003607e-3_f64) * t33524 * t34952 + F::cast_from(0.28234466758480466999e-3_f64) * t33414 * t34957 - F::cast_from(0.17135921299530705785e1_f64) * t33477 * t34961 - F::cast_from(0.28234466758480466999e-3_f64) * t8926 * t34966 + F::cast_from(0.57119737665102352616e0_f64) * t34969 * t8941 - F::cast_from(0.1859366460452550541e-3_f64) * t34972 * t8948 - F::cast_from(0.17347256376410398924e1_f64) * t33446 * t8217 + F::cast_from(0.17347256376410398924e1_f64) * t8932 * t33449 * t473 * t1828 - F::cast_from(0.17347256376410398924e1_f64) * t33436 * t34982 + F::cast_from(0.17347256376410398924e1_f64) * t33441 * t2150 * t473 * t1774 - F::cast_from(0.15058382271189582399e-2_f64) * t34991 * t8928;
    (t34989, t34990, t34991, t34994)
}
