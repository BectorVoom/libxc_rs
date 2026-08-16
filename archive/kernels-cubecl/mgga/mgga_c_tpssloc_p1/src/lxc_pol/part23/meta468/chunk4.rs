//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1378/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378<F: Float>(t5758: F, t10756: F, t10771: F, t10813: F, t10828: F, t14337: F, t17355: F, t21198: F, t21207: F, t21239: F, t21242: F, t21247: F, t2886: F, t2888: F, t2905: F, t2932: F, t42111: F, t42113: F, t42154: F, t42226: F, t42228: F, t4449: F, t49099: F, t49104: F, t49285: F, t5775: F, t5791: F, t5794: F, t60343: F, t60424: F, t76637: F, t77139: F, t77239: F, t77257: F, t77272: F, t77287: F, t77301: F, t924: F, t932: F, t951: F) -> (F, F) {
    let t77328 = t5758 * t5758;
    let t77343 = F::cast_from(0.82761620670837440481e4_f64) * t49285 * t21198 - F::cast_from(0.24828486201251232145e5_f64) * t42154 * t77239 * t10813 + F::cast_from(1.0_f64) * t924 * (t77257 + t77272 + t77287 + t77301) * t932 + F::cast_from(0.19964560303604640732e6_f64) * t42226 * t77239 * t42228 + F::cast_from(0.35089341735807877242e1_f64) * t17355 * t5791 + F::cast_from(0.10389515463408878255e3_f64) * t60343 * t5794 + F::cast_from(0.23392894490538584828e1_f64) * t4449 * t21239 + F::cast_from(0.4101607543286562663e4_f64) * t49104 * t21242 + F::cast_from(0.91082604192152556044e5_f64) * t42111 * t76637 * t42113 - F::cast_from(0.70178683471615754484e1_f64) * t60424 * t5775 - F::cast_from(0.4155806185363551302e3_f64) * t49099 * t21207 + F::cast_from(0.6233709278045326953e3_f64) * t10756 * t76637 * t2932 + F::cast_from(0.96491876992155210402e2_f64) * t2886 * t77328 * t2888 + F::cast_from(0.14035736694323150897e2_f64) * t14337 * t21247 - F::cast_from(0.14035736694323150897e2_f64) * t10828 * t76637 * t951 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t77139 * t951 - F::cast_from(24.0_f64) * t10771 * t77239 * t932;
    (t77328, t77343)
}
