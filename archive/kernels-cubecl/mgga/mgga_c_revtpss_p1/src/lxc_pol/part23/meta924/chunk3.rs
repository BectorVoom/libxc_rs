//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2991/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2991<F: Float>(t15932: F, t19826: F, t1065: F, t23598: F, t11630: F, t23829: F, t3172: F, t1011: F, t140: F, t24016: F, t1042: F, t11774: F, t11859: F, t15707: F, t15926: F, t19651: F, t19838: F, t19878: F, t19895: F, t19940: F, t20091: F, t24013: F, t24017: F, t3117: F, t3127: F, t3155: F, t3241: F, t42830: F, t4782: F, t4825: F, t53724: F, t53762: F, t53807: F, t53855: F, t6273: F, t6308: F, t65717: F, t66043: F, t67052: F, t79275: F, t906: F) -> F {
    let t79290 = t15932 * t19826;
    let t79301 = t1065 * t23598;
    let t79309 = t11630 * t3172 * t23829;
    let t79315 = t1011 * t140 * t24016;
    let t79331 = F::cast_from(0.57165357490759649296e-3_f64) * t66043 + F::cast_from(0.5081365110289746604e-3_f64) * t53724 - F::cast_from(0.42874018118069736972e-3_f64) * t79290 + F::cast_from(0.85748036236139473947e-3_f64) * t19878 * t19651 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t67052 * t4782 - F::cast_from(0.85748036236139473944e-3_f64) * t15707 * t19940 + F::cast_from(0.85748036236139473944e-3_f64) * t15707 * t19895 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t79301 * t906 - F::cast_from(0.42874018118069736972e-3_f64) * t65717 * t4825 + F::cast_from(0.85748036236139473947e-3_f64) * t79309 + F::cast_from(0.1270341277572436651e-3_f64) * t53762 - t3241 * t24017 / F::cast_from(18.0_f64) + t79315 / F::cast_from(144.0_f64) - F::cast_from(0.12862205435420921092e-2_f64) * t11859 * t3117 * t79275 * t3155 + F::cast_from(0.64311027177104605458e-3_f64) * t42830 * t24013 - F::cast_from(0.68598428988911579157e-2_f64) * t53807 * t6308 - F::cast_from(0.12862205435420921092e-2_f64) * t53855 * t6273 - F::cast_from(0.12862205435420921092e-2_f64) * t15926 * t19838 - F::cast_from(0.12862205435420921092e-2_f64) * t15926 * t20091;
    t79331
}
