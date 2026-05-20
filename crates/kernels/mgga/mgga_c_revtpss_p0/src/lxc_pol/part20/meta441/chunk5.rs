//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1680/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1680<F: Float>(t1204: F, t13126: F, t12722: F, t3566: F, t5462: F, t5477: F, t1209: F, t1284: F, t3727: F, t1234: F, t12621: F, t12706: F, t12719: F, t12769: F, t1281: F, t12975: F, t13130: F, t13133: F, t13161: F, t17864: F, t3568: F, t3666: F, t3670: F, t3756: F, t3759: F, t3763: F, t3769: F, t3783: F, t44552: F, t44832: F, t45385: F) -> F {
    let t45846 = t1204 * t13126;
    let t45852 = t3566 * t12722;
    let t45859 = t3566 * t5462;
    let t45863 = t3566 * t5477;
    let t45868 = t1209 * t1284 * t3727;
    let t45873 = F::cast_from(0.79025390195226139183e1_f64) * t3670 * t13133 * t3568 - F::cast_from(0.39512695097613069592e1_f64) * t12975 * t3763 - F::cast_from(0.26341796731742046395e1_f64) * t44832 * t1281 + F::cast_from(0.26341796731742046395e1_f64) * t45846 * t13130 - F::cast_from(0.26341796731742046395e1_f64) * t1234 * t3759 * t12621 + F::cast_from(0.15805078039045227836e2_f64) * t45852 * t12719 - F::cast_from(0.15805078039045227836e2_f64) * t45385 * t13161 - F::cast_from(0.26341796731742046395e1_f64) * t3666 * t12769 + F::cast_from(0.15805078039045227836e2_f64) * t45859 * t44552 * t3769 - F::cast_from(0.79025390195226139183e1_f64) * t45863 * t44552 * t3783 - F::cast_from(0.79025390195226139183e1_f64) * t45868 * t3756 - F::cast_from(0.79025390195226139184e1_f64) * t17864 * t12706;
    t45873
}
