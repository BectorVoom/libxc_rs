//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1005/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1005<F: Float>(t10034: F, t505: F, t3678: F, t4796: F, t1117: F, t4792: F, t2856: F, t511: F, t532: F, t2860: F, t1153: F, t1148: F, t10004: F, t10007: F, t10010: F, t10014: F, t10022: F, t10029: F, t1520: F, t1523: F, t1525: F, t2817: F, t2823: F, t2828: F, t2832: F, t2885: F, t2890: F, t3741: F, t3760: F, t7818: F, t7838: F, t7848: F, tau0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10035 = t505 * t10034;
    let t10036 = t3678 * t4796;
    let t10039 = t1117 * t10034;
    let t10042 = t3678 * t4792;
    let t10045 = t2856 * tau0;
    let t10046 = t511 * t10045;
    let t10049 = t532 * tau0;
    let t10050 = t2860 * t10049;
    let t10053 = t1153 * tau0;
    let t10054 = t1148 * t10053;
    let t10059 = -0.176e-3 * t2817 * t10004 + 0.176e-3 * t2823 * t10007 + 0.46933333333333333333e-3 * t3760 * t10010 + 0.144e-3 * t7838 * t10014 + 0.528e-3 * t2832 * t10007 + 0.46933333333333333333e-3 * t3741 * t10010 + 0.72e-3 * t7848 * t10022 + 0.1008e-2 * t7818 * t10014 - 0.528e-3 * t2828 * t10004 + 2.0 * t10029 * t1525 - t1520 * t2885 - t2890 * t1523 + 200.0 / 9.0 * t10035 * t10036 + 400.0 / 9.0 * t10039 * t10036 - 400.0 / 9.0 * t10039 * t10042 + 200.0 / 3.0 * t10046 * t10036 - 1000.0 / 3.0 * t10050 * t10042 + 400.0 * t10054 * t10036 - 400.0 * t10054 * t10042;
    (t10035, t10036, t10039, t10045, t10046, t10050, t10053, t10054, t10059)
}
