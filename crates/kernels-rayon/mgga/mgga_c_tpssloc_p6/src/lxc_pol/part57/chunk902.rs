//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 902/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk902(t193: f64, t201: f64, t7844: f64, t2053: f64, t40889: f64, t10143: f64, t2091: f64, t40590: f64, t111: f64, t7945: f64, t27992: f64, t7684: f64, t8944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92319 = t193 * t201 * t7844;
    let t92394 = t40889 * t2053;
    let t93000 = t7844 * t10143;
    let t93319 = t40590 * t2091;
    let t94170 = t7945 * t111;
    let t96686 = t27992 * t111;
    let t96797 = t7684 * t8944;
    (t92319, t92394, t93000, t93319, t94170, t96686, t96797)
}
