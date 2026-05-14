//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1368/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1368<F: Float>(t1815: F, t498: F, t1567: F, t1128: F, t2849: F, t4529: F, t11981: F, t2922: F, t4533: F, t7995: F, t11766: F, t11999: F, t24504: F, t24640: F, t24649: F, t24662: F, t27886: F, t2839: F, t2887: F, t2934: F, t2946: F, t2953: F, t2957: F, t33638: F, t33751: F, t4541: F, t4555: F, t4559: F, t4603: F, t7913: F, t8094: F) -> (F, F) {
    let t33770 = t498 * t1815;
    let t33771 = t33770 * t1567;
    let t33777 = t1128 * t4529 * t2849;
    let t33780 = t2922 * t11981;
    let t33783 = t4533 * t7995;
    let t33805 = -0.12e-1 * t8094 * t4555 - 0.256e-3 * t7913 * t4559 - 4032.0 * t27886 * t33751 - 0.256e-3 * t11766 * t2934 + 10000.0 / 81.0 * t33771 * t33638 - 4.0 * t2887 * t4603 + 0.36e0 * t24649 * t33777 + 0.176e0 * t2946 * t33780 + 0.176e0 * t2946 * t33783 + 0.108e1 * t24640 * t1128 * t4541 * t2839 + 0.378e1 * t24504 * t1128 * t4541 * t2849 + 0.88e0 * t2953 * t2922 * t11999 + 0.378e1 * t24504 * t1128 * t4529 * t2839 + 0.9072e1 * t24662 * t33777 + 0.1232e1 * t2957 * t33780;
    (t33783, t33805)
}
