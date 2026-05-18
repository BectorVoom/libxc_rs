//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1044/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1044<F: Float>(t1145: F, t9778: F, t1128: F, t3785: F, t1535: F, t2893: F, t3788: F, t513: F, t2889: F, t1540: F, t1543: F, t1552: F, t2868: F, t2875: F, t2881: F, t2903: F, t2922: F, t2927: F, t2940: F, t3706: F, t3714: F, t3778: F, t510: F, t7764: F, t9475: F, t9765: F, t9766: F, t9769: F, t9770: F, t9773: F) -> (F, F, F) {
    let t9779 = t1145 * t9778;
    let t9782 = t3785 * t1128;
    let t9785 = t1535 * t2893;
    let t9786 = t1145 * t9785;
    let t9793 = t3788 * t513;
    let t9796 = t1535 * t2889;
    let t9810 = F::new(24.0) * t9765 * t9766 - F::new(360.0) * t9769 * t9770 + F::new(504.0) * t9773 * t9766 - F::new(2.0) * t7764 * t1552 + F::new(21.0) * t2875 * t9779 - F::new(4.0) * t9782 * t3706 - F::new(2.0) * t2927 * t9786 - F::new(2.0) * t2927 * t9475 + F::new(3.0) * t2881 * t9779 - F::new(200.0) / F::new(9.0) * t9793 * t3714 + F::new(15.0) * t2868 * t1145 * t9796 - F::new(18.0) * t2922 * t9786 + F::new(6.0) * t510 * t3778 * t2893 - F::new(4.0) * t2940 * t1540 + F::new(30.0) * t2903 * t1543 * t2889;
    (t9782, t9793, t9810)
}
