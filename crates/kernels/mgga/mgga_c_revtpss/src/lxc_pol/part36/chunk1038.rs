//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1038/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1038<F: Float>(t19501: F, t23898: F, t3092: F, t6266: F, t19611: F, t357: F, t4781: F, t6100: F, t6092: F, t11703: F, t6096: F, t1011: F, t1063: F, t11737: F, t15618: F, t15712: F, t15732: F, t15750: F, t19786: F, t19827: F, t19867: F, t19883: F, t23874: F, t23878: F, t23886: F, t23892: F, t3091: F, t3127: F, t4834: F, t4892: F, t4899: F, t6268: F, t6331: F) -> F {
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
    let t23926 = F::new(7.0) / F::new(648.0) * t1011 * t23874 - t1011 * t23878 / F::new(36.0) + F::new(0.57165357490759649295e-3) * t19786 - F::new(0.95275595817932748825e-4) * t15712 - F::new(0.14291339372689912324e-3) * t15732 + F::new(0.14291339372689912324e-3) * t15750 - F::new(0.14291339372689912324e-2) * t1063 * t23886 - F::new(0.85748036236139473944e-3) * t4834 * t6331 + t11737 - F::new(0.42874018118069736972e-3) * t3127 * t23892 - F::new(0.42874018118069736972e-3) * t19827 + F::new(0.85748036236139473944e-3) * t15618 * t6268 + F::new(0.85748036236139473944e-3) * t4892 * t23900 - F::new(0.42874018118069736972e-3) * t4899 * t23904 + F::new(0.42874018118069736972e-3) * t3091 * t23908 + F::new(0.42874018118069736972e-3) * t3091 * t23913 + F::new(0.7145669686344956162e-3) * t3091 * t23917 - F::new(0.85748036236139473944e-3) * t3091 * t23921 + F::new(0.42874018118069736972e-3) * t19867 - F::new(0.57165357490759649295e-3) * t19883;
    t23926
}
