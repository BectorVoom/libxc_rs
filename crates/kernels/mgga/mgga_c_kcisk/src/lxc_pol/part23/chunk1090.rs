//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1090/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1090<F: Float>(t524: F, t21713: F, t21883: F, t1589: F, t1586: F, t4374: F, t6581: F, t1591: F, t6204: F, t1576: F, t6453: F, t2318: F, t4416: F, t1571: F, t6449: F, t6450: F, t2317: F, t4509: F) -> (F, F, F, F, F, F, F, F, F) {
    let t536 = 0.0 < t524;
    let t21884 = t21713 + t21883;
    let t21886 = piecewise3(t536, t21884, -t21884);
    let t21887 = t1589 * t21886;
    let t21888 = t1586 * t21887;
    let t21895 = t4374 * t6581;
    let t21896 = t21895 * t1591;
    let t21897 = t6204 * t21896;
    let t21900 = t6453 * t1576;
    let t21902 = t2318 * t4416;
    let t21904 = t6449 * t1571;
    let t21908 = 0.17990788716177317213e-1 * t6450 * t1576;
    let t21909 = t2317 * t4509;
    (t21884, t21886, t21888, t21897, t21900, t21902, t21904, t21908, t21909)
}
