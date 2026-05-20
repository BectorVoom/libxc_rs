//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3226/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3226<F: Float>(t45785: F, t460: F, t487: F, t13043: F, t43350: F, t45832: F, t5219: F, t5462: F, t1204: F, t12714: F, t12723: F, t12753: F, t17172: F, t17175: F, t17808: F, t17861: F, t17905: F, t17945: F, t21452: F, t3552: F, t3603: F, t3666: F, t3746: F, t3774: F, t45707: F, t45852: F, t45868: F, t471: F, t5459: F, t5466: F, t58921: F) -> F {
    let t59730 = t460 * t45785 * t487;
    let t59731 = t43350 * t13043;
    let t59737 = t460 * t45832 * t487;
    let t59749 = t5219 * t5462;
    let t59762 = -F::cast_from(0.19756347548806534796e1_f64) * t12723 * t17905 + F::cast_from(0.39512695097613069591e1_f64) * t21452 * t12714 + F::cast_from(0.92196288561097162379e1_f64) * t59730 * t58921 * t59731 * t3603 - F::cast_from(0.65854491829355115987e0_f64) * t59737 * t58921 * t59731 * t471 + F::cast_from(0.39512695097613069591e1_f64) * t3552 * t5462 * t5466 - F::cast_from(0.19756347548806534796e1_f64) * t45868 * t5459 - F::cast_from(0.19756347548806534796e1_f64) * t3666 * t17175 - F::cast_from(0.39512695097613069591e1_f64) * t59749 * t12753 + F::cast_from(0.39512695097613069591e1_f64) * t17861 * t3774 + F::cast_from(0.39512695097613069591e1_f64) * t45707 * t17945 + F::cast_from(0.39512695097613069591e1_f64) * t45852 * t17945 + F::cast_from(0.19756347548806534796e1_f64) * t1204 * t17808 + F::cast_from(0.19756347548806534796e1_f64) * t3746 * t17172;
    t59762
}
