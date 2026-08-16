//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2272/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2272(t15285: f64, t4889: f64, t17686: f64, t44505: f64, t15363: f64, t1174: f64, t15281: f64, t18549: f64, t18554: f64, t11570: f64, t17635: f64, t11583: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t65008 = t4889 * t15285;
    let t65018 = t44505 * t17686;
    let t65023 = t4889 * t15363;
    let t65035 = t1174 * t15281 * t18549;
    let t65041 = t1174 * t15281 * t18554;
    let t65056 = t11570 * t17635;
    let t65077 = t11583 * t17635;
    (t65008, t65018, t65023, t65035, t65041, t65056, t65077)
}
