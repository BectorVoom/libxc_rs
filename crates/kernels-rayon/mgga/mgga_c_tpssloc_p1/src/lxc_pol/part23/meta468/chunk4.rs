//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1378/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1378(t5758: f64, t10756: f64, t10771: f64, t10813: f64, t10828: f64, t14337: f64, t17355: f64, t21198: f64, t21207: f64, t21239: f64, t21242: f64, t21247: f64, t2886: f64, t2888: f64, t2905: f64, t2932: f64, t42111: f64, t42113: f64, t42154: f64, t42226: f64, t42228: f64, t4449: f64, t49099: f64, t49104: f64, t49285: f64, t5775: f64, t5791: f64, t5794: f64, t60343: f64, t60424: f64, t76637: f64, t77139: f64, t77239: f64, t77257: f64, t77272: f64, t77287: f64, t77301: f64, t924: f64, t932: f64, t951: f64) -> (f64, f64) {
    let t77328 = t5758 * t5758;
    let t77343 = 0.82761620670837440481e4_f64 * t49285 * t21198 - 0.24828486201251232145e5_f64 * t42154 * t77239 * t10813 + 1.0_f64 * t924 * (t77257 + t77272 + t77287 + t77301) * t932 + 0.19964560303604640732e6_f64 * t42226 * t77239 * t42228 + 0.35089341735807877242e1_f64 * t17355 * t5791 + 0.10389515463408878255e3_f64 * t60343 * t5794 + 0.23392894490538584828e1_f64 * t4449 * t21239 + 0.4101607543286562663e4_f64 * t49104 * t21242 + 0.91082604192152556044e5_f64 * t42111 * t76637 * t42113 - 0.70178683471615754484e1_f64 * t60424 * t5775 - 0.4155806185363551302e3_f64 * t49099 * t21207 + 0.6233709278045326953e3_f64 * t10756 * t76637 * t2932 + 0.96491876992155210402e2_f64 * t2886 * t77328 * t2888 + 0.14035736694323150897e2_f64 * t14337 * t21247 - 0.14035736694323150897e2_f64 * t10828 * t76637 * t951 - 0.35089341735807877242e1_f64 * t2905 * t77139 * t951 - 24.0_f64 * t10771 * t77239 * t932;
    (t77328, t77343)
}
