//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1395/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1395(t12605: f64, t18120: f64, t1889: f64, t4463: f64, t4440: f64, t1607: f64, t5713: f64, t1610: f64, t5477: f64, t16082: f64, t6159: f64, t1369: f64, t531: f64, t617: f64) -> (f64, f64, f64, f64, f64) {
    let t18121 = t12605 * t18120;
    let t18124 = t1889 * t4463;
    let t18125 = t4440 * t18124;
    let t18128 = t5713 * t1607;
    let t18129 = t5477 * t1610;
    let t18130 = t18128 * t18129;
    let t18133 = t6159 * t16082;
    let t18137 = t1369 * t617 * t531;
    (t18121, t18125, t18130, t18133, t18137)
}
