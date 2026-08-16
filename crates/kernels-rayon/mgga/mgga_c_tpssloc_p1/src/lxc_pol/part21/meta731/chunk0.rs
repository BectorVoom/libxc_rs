//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2587/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587(t3447: f64, t44583: f64, t461: f64, t4729: f64, t15418: f64, t1714: f64, t11571: f64, t14736: f64, t15419: f64, t14165: f64, t44505: f64, t11557: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52057 = t3447 * t44583 * t461 * t4729;
    let t52059 = t15418 * t1714;
    let t52061 = t3447 * t52059 * t11571;
    let t52064 = t3447 * t15419 * t14736;
    let t52066 = t44505 * t14165;
    let t52074 = t4889 * t11557;
    (t52057, t52059, t52061, t52064, t52066, t52074)
}
