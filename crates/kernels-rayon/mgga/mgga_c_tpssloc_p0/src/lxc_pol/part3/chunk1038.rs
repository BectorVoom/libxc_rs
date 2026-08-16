//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1038/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1038(t776: f64, t868: f64, t13110: f64, t13112: f64, t13114: f64, t13117: f64, t13118: f64, t13121: f64, t13122: f64, t13125: f64, t13129: f64, t13132: f64, t13135: f64, t13136: f64, t13137: f64, t2379: f64, t2522: f64, t4307: f64, t4310: f64, t4314: f64, t9853: f64, t9859: f64, t9894: f64, t9907: f64, t9921: f64) -> f64 {
    let t13487 = t776 * t868;
    let t13491 = -6.0_f64 * t13487 * t2522 * t4307 + 6.0_f64 * t2379 * t4310 * t4314 + t13110 - t13112 - t13114 + t13117 + t13118 - t13121 - t13122 + t13125 + t13129 + t13132 + t13135 + t13136 + t13137 + t9853 + t9859 - t9894 + t9907 - t9921;
    t13491
}
