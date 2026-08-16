//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1181/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1181(t2651: f64, t7671: f64, t26654: f64, t838: f64, t26633: f64, t26652: f64, t26420: f64, t1505: f64, t27489: f64, t12286: f64, t491: f64, t990: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93817 = t2651 * t7671;
    let t93826 = t838 * t26654;
    let t93848 = 3.0_f64 * t26633;
    let t93849 = 3.0_f64 * t26652;
    let t93852 = 12.0_f64 * t26420;
    let t94197 = t27489 * t1505;
    let t94208 = t12286 * t491 * t990;
    (t93817, t93826, t93848, t93849, t93852, t94197, t94208)
}
