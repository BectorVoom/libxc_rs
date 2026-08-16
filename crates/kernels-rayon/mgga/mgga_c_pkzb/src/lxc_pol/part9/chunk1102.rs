//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1102/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1102(t2070: f64, t2082: f64, t5674: f64, t771: f64, t310: f64, t5999: f64, t5989: f64, t751: f64, t2021: f64, t296: f64, t2030: f64, t5913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18234 = t2082 * t2070;
    let t18236 = t771 * t5674;
    let t18258 = 1.0_f64 / t5999 / t310;
    let t18284 = t751 * t5989;
    let t18290 = 1.0_f64 / t2021 / t296;
    let t18331 = t2030 * t5913;
    (t18234, t18236, t18258, t18284, t18290, t18331)
}
