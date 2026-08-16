//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1072/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1072(t1009: f64, t2844: f64, t2630: f64, t4939: f64, t10454: f64, t922: f64, t4947: f64, t2635: f64, t3203: f64, t7718: f64, t1020: f64, t4555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26695 = t1009 * t2844;
    let t26696 = t26695 * t2630;
    let t26697 = t4939 * t26696;
    let t26702 = t10454 * t922;
    let t26703 = t4947 * t26702;
    let t26706 = t3203 * t2635;
    let t26707 = t7718 * t26706;
    let t26708 = t1020 * t26707;
    let t26710 = t4555 * t2630;
    (t26695, t26696, t26697, t26702, t26703, t26706, t26707, t26708, t26710)
}
