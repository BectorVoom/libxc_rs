//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1364/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1364<F: Float>(t24192: F, t4619: F, t1795: F, t2893: F, t4558: F, t516: F, t7917: F, t1815: F, t509: F, t1567: F, t1616: F, t4097: F, t513: F, t10080: F, t11789: F, t11794: F, t11909: F, t28161: F, t28634: F, t28648: F, t28705: F, t2922: F, t33082: F, t33187: F, t33190: F, t33198: F, t33205: F, t33208: F, t33595: F, t3760: F, t3799: F, t7832: F, t7838: F, t7848: F, t7854: F, t9718: F, sigma2: F) -> (F, F, F) {
    let t33599 = t24192 * t4619;
    let t33606 = t1795 * t2893;
    let t33607 = t33606 * t4558;
    let t33619 = t516 * t7917 * sigma2;
    let t33635 = t509 * t1815;
    let t33636 = t33635 * t1567;
    let t33637 = t4097 * t1616;
    let t33638 = t33637 * t513;
    let t33641 = -0.2112e1 * t7854 * t33599 - 0.528e-3 * t7838 * t33082 + 0.2304e-5 * t7838 * t33198 + 0.11264e-4 * t11789 * t33607 - 0.264e-2 * t7848 * t11794 * t2922 - 0.432e1 * t28634 * t3799 * t9718 - 0.528e-3 * t7838 * t33208 + 0.384e0 * t33619 * t33187 - 0.17208888888888888889e-2 * t3760 * t33190 + 0.11264e-4 * t11909 * t33607 - 0.288e0 * t28161 * t33595 - 0.23466666666666666667e0 * t7832 * t33599 - 0.192e0 * t28705 * t33205 + 0.288e1 * t28648 * t10080 * t9718 + 10000.0 / 81.0 * t33636 * t33638;
    (t33637, t33638, t33641)
}
