//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3000/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3000(t1042: f64, t1063: f64, t1068: f64, t15817: f64, t15850: f64, t19800: f64, t23834: f64, t23852: f64, t23886: f64, t3106: f64, t3188: f64, t42648: f64, t42716: f64, t42740: f64, t42745: f64, t4879: f64, t54148: f64, t54537: f64, t6302: f64, t6331: f64, t66547: f64, t66551: f64, t78785: f64, t79553: f64, t79559: f64, t79564: f64, t79575: f64, t79580: f64) -> f64 {
    let t79588 = -0.57165357490759649296e-3_f64 * t79553 - 0.14291339372689912324e-2_f64 * t3188 * t23886 - 0.85748036236139473944e-3_f64 * t15850 * t6331 + 0.14291339372689912324e-3_f64 * t79559 * t1068 + 0.85748036236139473947e-3_f64 * t79564 - t54148 + 5.0_f64 / 3888.0_f64 * t42716 - 5.0_f64 / 486.0_f64 * t42740 - t42745 + 0.64311027177104605458e-3_f64 * t15817 * t6302 + 0.64311027177104605458e-3_f64 * t4879 * t19800 + 0.68598428988911579157e-2_f64 * t42648 * t23834 - 0.85748036236139473947e-3_f64 * t79575 + 0.45732285992607719437e-2_f64 * t3106 * t23852 - 0.57165357490759649296e-3_f64 * t79580 - 0.76220476654346199062e-2_f64 * t1063 * t1042 * t54537 * t78785 + t66547 / 216.0_f64 + t66551 / 54.0_f64;
    t79588
}
