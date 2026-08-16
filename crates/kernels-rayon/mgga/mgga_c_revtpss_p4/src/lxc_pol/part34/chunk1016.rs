//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1016/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1016(t19501: f64, t23898: f64, t3092: f64, t6266: f64, t19611: f64, t357: f64, t4781: f64, t6100: f64, t6092: f64, t11703: f64, t6096: f64, t1011: f64, t1063: f64, t11737: f64, t15618: f64, t15712: f64, t15732: f64, t15750: f64, t19786: f64, t19827: f64, t19867: f64, t19883: f64, t23874: f64, t23878: f64, t23886: f64, t23892: f64, t3091: f64, t3127: f64, t4834: f64, t4892: f64, t4899: f64, t6268: f64, t6331: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23899 = t19501 * t23898;
    let t23900 = t3092 * t23899;
    let t23903 = t19501 * t6266;
    let t23904 = t3092 * t23903;
    let t23907 = t19611 * t6266;
    let t23908 = t3092 * t23907;
    let t23911 = t4781 * t357;
    let t23912 = t6100 * t23911;
    let t23913 = t3092 * t23912;
    let t23916 = t6092 * t23911;
    let t23917 = t11703 * t23916;
    let t23920 = t6096 * t23911;
    let t23921 = t3092 * t23920;
    let t23926 = 7.0_f64 / 648.0_f64 * t1011 * t23874 - t1011 * t23878 / 36.0_f64 + 0.57165357490759649295e-3_f64 * t19786 - 0.95275595817932748825e-4_f64 * t15712 - 0.14291339372689912324e-3_f64 * t15732 + 0.14291339372689912324e-3_f64 * t15750 - 0.14291339372689912324e-2_f64 * t1063 * t23886 - 0.85748036236139473944e-3_f64 * t4834 * t6331 + t11737 - 0.42874018118069736972e-3_f64 * t3127 * t23892 - 0.42874018118069736972e-3_f64 * t19827 + 0.85748036236139473944e-3_f64 * t15618 * t6268 + 0.85748036236139473944e-3_f64 * t4892 * t23900 - 0.42874018118069736972e-3_f64 * t4899 * t23904 + 0.42874018118069736972e-3_f64 * t3091 * t23908 + 0.42874018118069736972e-3_f64 * t3091 * t23913 + 0.7145669686344956162e-3_f64 * t3091 * t23917 - 0.85748036236139473944e-3_f64 * t3091 * t23921 + 0.42874018118069736972e-3_f64 * t19867 - 0.57165357490759649295e-3_f64 * t19883;
    (t23900, t23904, t23908, t23913, t23917, t23921, t23926)
}
