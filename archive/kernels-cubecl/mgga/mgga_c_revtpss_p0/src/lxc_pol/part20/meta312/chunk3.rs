//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1218/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1218<F: Float>(t1214: F, t3601: F, t3769: F, t1209: F, t5477: F, t3783: F, t12690: F, t12699: F, t12702: F, t12706: F, t12709: F, t12714: F, t12717: F, t12719: F, t12723: F, t12727: F, t12734: F, t12737: F, t12741: F, t12744: F, t12748: F, t12751: F, t1285: F, t1288: F, t1291: F, t3552: F, t3670: F, t3746: F, t3755: F, t3756: F, t3770: F, t3774: F, t3784: F, t490: F, t5463: F, t5478: F) -> (F, F, F, F) {
    let t12752 = t1214 * t3601;
    let t12753 = t12752 * t3769;
    let t12756 = t1209 * t5477;
    let t12757 = t12752 * t3783;
    let t12766 = F::cast_from(0.19756347548806534796e1_f64) * t12699 * t1288 + F::cast_from(0.39512695097613069591e1_f64) * t12702 * t3770 - F::cast_from(0.19756347548806534796e1_f64) * t5478 * t12706 - F::cast_from(0.39512695097613069591e1_f64) * t12709 * t3756 + F::cast_from(0.39512695097613069591e1_f64) * t5463 * t12714 + F::cast_from(0.39512695097613069591e1_f64) * t12717 * t12719 - F::cast_from(0.39512695097613069591e1_f64) * t12723 * t3756 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t12727 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t12734 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t12737 + F::cast_from(0.19756347548806534796e1_f64) * t1285 * t12741 - F::cast_from(0.19756347548806534796e1_f64) * t12744 * t3784 - F::cast_from(0.19756347548806534796e1_f64) * t3755 * t12748 - F::cast_from(0.39512695097613069591e1_f64) * t12751 * t12753 + F::cast_from(0.19756347548806534796e1_f64) * t12756 * t12757 + F::cast_from(0.65854491829355115987e0_f64) * t12690 * t490 + F::cast_from(0.19756347548806534796e1_f64) * t3552 * t1291 + F::cast_from(0.39512695097613069591e1_f64) * t3746 * t3774;
    (t12753, t12756, t12757, t12766)
}
