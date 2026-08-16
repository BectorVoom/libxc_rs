//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1680/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1680(t1204: f64, t13126: f64, t12722: f64, t3566: f64, t5462: f64, t5477: f64, t1209: f64, t1284: f64, t3727: f64, t1234: f64, t12621: f64, t12706: f64, t12719: f64, t12769: f64, t1281: f64, t12975: f64, t13130: f64, t13133: f64, t13161: f64, t17864: f64, t3568: f64, t3666: f64, t3670: f64, t3756: f64, t3759: f64, t3763: f64, t3769: f64, t3783: f64, t44552: f64, t44832: f64, t45385: f64) -> f64 {
    let t45846 = t1204 * t13126;
    let t45852 = t3566 * t12722;
    let t45859 = t3566 * t5462;
    let t45863 = t3566 * t5477;
    let t45868 = t1209 * t1284 * t3727;
    let t45873 = 0.79025390195226139183e1_f64 * t3670 * t13133 * t3568 - 0.39512695097613069592e1_f64 * t12975 * t3763 - 0.26341796731742046395e1_f64 * t44832 * t1281 + 0.26341796731742046395e1_f64 * t45846 * t13130 - 0.26341796731742046395e1_f64 * t1234 * t3759 * t12621 + 0.15805078039045227836e2_f64 * t45852 * t12719 - 0.15805078039045227836e2_f64 * t45385 * t13161 - 0.26341796731742046395e1_f64 * t3666 * t12769 + 0.15805078039045227836e2_f64 * t45859 * t44552 * t3769 - 0.79025390195226139183e1_f64 * t45863 * t44552 * t3783 - 0.79025390195226139183e1_f64 * t45868 * t3756 - 0.79025390195226139184e1_f64 * t17864 * t12706;
    t45873
}
