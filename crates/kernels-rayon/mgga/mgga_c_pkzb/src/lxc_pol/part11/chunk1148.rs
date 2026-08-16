//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1148/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1148(t10063: f64, t8273: f64, t10116: f64, t3174: f64, t68: f64, t931: f64, t9795: f64, t10071: f64, t3206: f64, t926: f64, t10102: f64, t8450: f64) -> (f64, f64, f64, f64, f64) {
    let t26986 = t10063 * t8273;
    let t26995 = t3174 * t68 * t10116;
    let t27001 = t931 * t9795;
    let t27007 = t3206 * t926 * t10071;
    let t27014 = t8450 * t926 * t10102;
    (t26986, t26995, t27001, t27007, t27014)
}
