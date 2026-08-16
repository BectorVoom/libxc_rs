//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1012/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1012(t5730: f64, t7764: f64, t2083: f64, t7757: f64, t13009: f64, t12969: f64, t12941: f64, t30233: f64, t26: f64, t1186: f64, t30238: f64, t30290: f64, t3661: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30613 = t5730 * t7764;
    let t30616 = t7757 * t2083;
    let t30617 = t13009 * t30616;
    let t30623 = t12969 * t30616;
    let t30625 = t12941 * t30233;
    let t30626 = t26 * t30625;
    let t30628 = t1186 * t30238;
    let t30629 = t26 * t30628;
    let t30631 = t3661 * t30290;
    (t30613, t30616, t30617, t30623, t30626, t30629, t30631)
}
