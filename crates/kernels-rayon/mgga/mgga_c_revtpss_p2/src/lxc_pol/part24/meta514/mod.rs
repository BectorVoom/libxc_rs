//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta514 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta514(t1065: f64, t23598: f64, t11630: f64, t23829: f64, t3172: f64, t1011: f64, t140: f64, t24016: f64, t11710: f64, t23907: f64, t3091: f64, t23912: f64, t1668: f64, t905: f64, t11774: f64, t53391: f64, t6267: f64, t19968: f64, t4817: f64, t20054: f64, t4834: f64, t19882: f64, t1062: f64, t23960: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79301, t79309, t79315, t79428, t79439) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1532(t1065, t23598, t11630, t23829, t3172, t1011, t140, t24016, t11710, t23907, t3091, t23912);
        let (t79450, t79474, t79546, t79548, t79553, t79559) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1533(t1668, t905, t11774, t53391, t6267, t19968, t4817, t20054, t4834, t19882, t1062, t23960);
    (t79301, t79309, t79315, t79428, t79439, t79450, t79474, t79546, t79548, t79553, t79559)
}
