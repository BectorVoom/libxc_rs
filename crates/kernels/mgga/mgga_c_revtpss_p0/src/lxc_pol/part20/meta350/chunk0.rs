//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1278/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1278<F: Float>(t17879: F, t460: F, t1269: F, t3766: F, t13126: F, t487: F, t1204: F, t5462: F, t3566: F, t488: F, t1209: F, t1045: F, t999: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17880 = t460 * t17879;
    let t17887 = t3766 * t1269;
    let t17888 = t460 * t17887;
    let t17948 = t13126 * t487;
    let t17949 = t460 * t17948;
    let t17955 = t1204 * t5462;
    let t17973 = t3566 * t488;
    let t17986 = t1209 * t488;
    let t19620 = t1045 * t999;
    (t17880, t17887, t17888, t17948, t17949, t17955, t17973, t17986, t19620)
}
