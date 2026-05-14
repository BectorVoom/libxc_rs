//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 996/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk996<F: Float>(t1440: F, t7831: F, t1450: F, t1415: F, t1411: F, t26411: F, t3776: F, t1340: F, t3764: F, t7740: F, t1339: F, t2075: F, t5996: F, t13377: F, t3482: F, t1056: F, t2231: F) -> (F, F, F, F, F, F, F) {
    let t26796 = t7831 * t1440;
    let t26797 = t1450 * t26796;
    let t26798 = t1415 * t26797;
    let t26799 = t1411 * t26798;
    let t26801 = t3776 * t26411;
    let t26802 = t1340 * t26801;
    let t26803 = t1411 * t26802;
    let t26805 = t3764 * t7740;
    let t26806 = t1340 * t26805;
    let t26807 = t1339 * t26806;
    let t26809 = t2075 * t5996;
    let t26810 = t13377 * t26809;
    let t26811 = t3482 * t26810;
    let t26813 = t2231 * t1056;
    (t26796, t26799, t26803, t26807, t26809, t26811, t26813)
}
