//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2290/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2290(t15689: f64, t4889: f64, t1174: f64, t135: f64, t18996: f64, t15743: f64, t5024: f64, t18363: f64, t3577: f64, t45124: f64, t11697: f64, t18359: f64) -> (f64, f64, f64, f64, f64) {
    let t66273 = t4889 * t15689;
    let t66276 = t1174 * t135 * t18996;
    let t66324 = t5024 * t15743;
    let t66334 = t3577 * t45124 * t18363;
    let t66337 = t3577 * t11697 * t18359;
    (t66273, t66276, t66324, t66334, t66337)
}
