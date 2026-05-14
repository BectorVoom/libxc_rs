//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1244/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1244<F: Float>(t5257: F, t8983: F, t1721: F, t24210: F, t1719: F, t8914: F, t6892: F, t8959: F, t6897: F, t6979: F, t2593: F, t2646: F, t6970: F, t1634: F, t16343: F, t17054: F, t17067: F, t1733: F, t179: F, t24070: F, t2592: F, t2600: F, t2645: F, t5244: F, t6859: F, t6896: F, t6939: F, t6944: F, t8953: F, t8971: F, t9003: F) -> (F, F, F, F, F, F, F) {
    let t24402 = t5257 * t8983;
    let t24407 = t24210 * t1721;
    let t24415 = t8914 * t1719;
    let t24421 = t6892 * t8959;
    let t24431 = t6897 * t6979;
    let t24435 = t2593 * t6979;
    let t24443 = t2646 * t6970;
    let t24459 = 0.45351183609335988442e-1 * t17054 - 0.12004725073059526352e-1 * t24421 + 0.10289764348336736874e-1 * t17067 * t179 * t8953 * t6944 - 0.10289764348336736874e-1 * t5244 * t179 * t8914 * t6944 - 0.51448821741683684367e-2 * t6896 * t179 * t24431 + 0.51448821741683684367e-2 * t2592 * t179 * t24435 + 0.85748036236139473945e-2 * t16343 * t179 * t8914 * t1634 - 0.85748036236139473944e-3 * t2645 * t179 * t24443 + 0.34299214494455789578e-2 * t1733 * t179 * t2600 * t24070 + 0.17149607247227894789e-2 * t1733 * t179 * t8971 * t6939 + 0.17149607247227894789e-2 * t1733 * t179 * t6859 * t9003;
    (t24402, t24407, t24415, t24431, t24435, t24443, t24459)
}
