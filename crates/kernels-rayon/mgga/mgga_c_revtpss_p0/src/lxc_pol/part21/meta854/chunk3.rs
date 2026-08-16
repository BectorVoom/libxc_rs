//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3226/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3226(t45785: f64, t460: f64, t487: f64, t13043: f64, t43350: f64, t45832: f64, t5219: f64, t5462: f64, t1204: f64, t12714: f64, t12723: f64, t12753: f64, t17172: f64, t17175: f64, t17808: f64, t17861: f64, t17905: f64, t17945: f64, t21452: f64, t3552: f64, t3603: f64, t3666: f64, t3746: f64, t3774: f64, t45707: f64, t45852: f64, t45868: f64, t471: f64, t5459: f64, t5466: f64, t58921: f64) -> f64 {
    let t59730 = t460 * t45785 * t487;
    let t59731 = t43350 * t13043;
    let t59737 = t460 * t45832 * t487;
    let t59749 = t5219 * t5462;
    let t59762 = -0.19756347548806534796e1_f64 * t12723 * t17905 + 0.39512695097613069591e1_f64 * t21452 * t12714 + 0.92196288561097162379e1_f64 * t59730 * t58921 * t59731 * t3603 - 0.65854491829355115987e0_f64 * t59737 * t58921 * t59731 * t471 + 0.39512695097613069591e1_f64 * t3552 * t5462 * t5466 - 0.19756347548806534796e1_f64 * t45868 * t5459 - 0.19756347548806534796e1_f64 * t3666 * t17175 - 0.39512695097613069591e1_f64 * t59749 * t12753 + 0.39512695097613069591e1_f64 * t17861 * t3774 + 0.39512695097613069591e1_f64 * t45707 * t17945 + 0.39512695097613069591e1_f64 * t45852 * t17945 + 0.19756347548806534796e1_f64 * t1204 * t17808 + 0.19756347548806534796e1_f64 * t3746 * t17172;
    t59762
}
