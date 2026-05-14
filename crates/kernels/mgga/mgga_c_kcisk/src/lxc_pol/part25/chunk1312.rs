//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1312/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1312<F: Float>(t11245: F, t1772: F, t2447: F, t32889: F, t9932: F, t3805: F, t9960: F, t17733: F, t7261: F, t9650: F, t1799: F, t33048: F, t34093: F, t33040: F, t34107: F, t648: F, t64908: F) -> (F, F, F, F, F, F, F) {
    let t116882 = t11245 * t2447 * t1772;
    let t116886 = t9932 * t32889;
    let t116888 = t3805 * t9960;
    let t116891 = t7261 * t9650 * t17733;
    let t116895 = t1799 * t34093 * t33048;
    let t116900 = t1799 * t34107 * t33040;
    let t116903 = t64908 * t648 * t1772;
    (t116882, t116886, t116888, t116891, t116895, t116900, t116903)
}
