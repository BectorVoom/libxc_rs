//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2986/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2986<F: Float>(t1045: F, t1053: F, t11672: F, t11927: F, t15618: F, t19572: F, t19620: F, t19716: F, t19731: F, t19738: F, t19741: F, t19873: F, t20066: F, t20070: F, t23823: F, t23837: F, t23921: F, t23960: F, t24009: F, t3117: F, t3169: F, t375: F, t42765: F, t43291: F, t4899: F, t53432: F, t53437: F, t53926: F, t6263: F, t65738: F, t79107: F, t79112: F, t79116: F, t79139: F, t79141: F) -> F {
    let t79151 = F::cast_from(0.57165357490759649296e-3_f64) * t79107 - F::cast_from(0.11433071498151929859e-2_f64) * t3169 * t23823 + F::cast_from(0.14291339372689912324e-3_f64) * t79112 + F::cast_from(0.45732285992607719437e-2_f64) * t53926 * t6263 + F::cast_from(0.12862205435420921092e-2_f64) * t11927 * t3117 * t79116 * t1045 + F::cast_from(0.68598428988911579157e-2_f64) * t42765 * t24009 + F::cast_from(0.12862205435420921092e-2_f64) * t19738 * t20066 - F::cast_from(0.64311027177104605458e-3_f64) * t19741 * t20070 + F::cast_from(0.57165357490759649295e-3_f64) * t65738 + F::cast_from(0.85748036236139473944e-3_f64) * t15618 * t19731 - F::cast_from(0.38586616306262763276e-2_f64) * t43291 * t3117 * t23837 * t19620 - t53432 - F::cast_from(0.85748036236139473944e-3_f64) * t15618 * t19873 + F::cast_from(0.45732285992607719437e-2_f64) * t11672 * t23921 - F::cast_from(0.57165357490759649296e-3_f64) * t79139 + F::cast_from(0.14291339372689912324e-3_f64) * t79141 - F::cast_from(0.11433071498151929859e-2_f64) * t23960 * t1053 * t375 - F::cast_from(0.95275595817932748827e-4_f64) * t53437 - F::cast_from(0.64311027177104605458e-3_f64) * t4899 * t3117 * t19572 * t19716;
    t79151
}
