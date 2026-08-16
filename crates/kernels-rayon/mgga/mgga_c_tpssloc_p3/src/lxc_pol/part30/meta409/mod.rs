//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1548;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1549;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta409(t1060: f64, t18088: f64, t1022: f64, t360: f64, t6739: f64, t5928: f64, t1049: f64, t5866: f64, t11066: f64, t3201: f64, t4649: f64, t1629: f64, t11060: f64, t4684: f64, t5936: f64, t4673: f64, t1058: f64, t1061: f64, t11034: f64, t11037: f64, t11046: f64, t11059: f64, t11065: f64, t14618: f64, t14651: f64, t1630: f64, t18081: f64, t18083: f64, t18086: f64, t3180: f64, t3186: f64, t3200: f64, t4674: f64, t5929: f64, t5937: f64, t5939: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t18089, t18093, t18094, t18099, t18100, t18103, t18104, t18107, t18108) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1548(t1060, t18088, t1022, t360, t6739, t5928, t1049, t5866, t11066, t3201, t4649, t1629);
        let (t18111, t18124) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1549(t1022, t11060, t5928, t4684, t5936, t4673, t1058, t1061, t11034, t11037, t11046, t11059, t11065, t14618, t14651, t1630, t18081, t18083, t18086, t18089, t18094, t18100, t18104, t18108, t3180, t3186, t3200, t4674, t5929, t5937, t5939);
    (t18093, t18099, t18103, t18107, t18111, t18124)
}
