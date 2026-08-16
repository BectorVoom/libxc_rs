//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1943/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1943(t23127: f64, t5628: f64, t16985: f64, t6621: f64, t1516: f64, t87321: f64, t25068: f64, t4261: f64, t5624: f64, t23133: f64, t87340: f64, t16673: f64, t6620: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98818 = t23127 * t5628;
    let t98820 = t6621 * t16985;
    let t98822 = t87321 * t1516;
    let t98824 = t25068 * t4261;
    let t98826 = t23127 * t5624;
    let t98828 = t23133 * t5624;
    let t98830 = t87340 * t1516;
    let t98832 = t16673 * t6620;
    (t98818, t98820, t98822, t98824, t98826, t98828, t98830, t98832)
}
