//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 410/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk410<F: Float>(t109: F, t1815: F, t1567: F, t320: F, t891: F, t646: F, t663: F, t105: F, t121: F, t1795: F, t1801: F, t1805: F, t1806: F, t1808: F, t1812: F, t650: F, t655: F, t659: F, t96: F) -> (F, F, F, F) {
    let t1816 = t109 * t1815;
    let t1817 = t1567 * t320;
    let t1820 = t891 * t1815;
    let t1821 = t1820 * t1567;
    let t1824 = t663 * t646;
    let t1827 = 0.39111111111111111112e-1 * t96 * t1795 * t105 - 0.38400000000000000001e-3 * t650 * t1801 * t655 + 0.91022222222222222228e-6 * t1805 * t1806 * t1808 - 40.0 / 9.0 * t659 * t1812 + 50.0 / 9.0 * t1816 * t1817 + 50.0 / 9.0 * t121 * t1821 - 40.0 / 9.0 * t121 * t1824;
    (t1817, t1821, t1824, t1827)
}
