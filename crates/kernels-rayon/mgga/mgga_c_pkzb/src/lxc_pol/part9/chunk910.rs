//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 910/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk910(t1034: f64, t1753: f64, t164: f64, t179: f64, t1721: f64, t2639: f64, t600: f64, t2593: f64, t2602: f64, t5257: f64, t1020: f64, t1719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6859 = t1034 * t1753;
    let t6860 = t6859 * t164;
    let t6861 = t179 * t6860;
    let t6864 = t2639 * t1721;
    let t6865 = t6864 * t600;
    let t6866 = t179 * t6865;
    let t6869 = t2593 * t1753;
    let t6870 = t179 * t6869;
    let t6873 = t5257 * t2602;
    let t6875 = t1020 * t1719;
    (t6859, t6860, t6861, t6864, t6865, t6866, t6869, t6870, t6873, t6875)
}
