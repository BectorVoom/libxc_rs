//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1304/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1304(t2845: f64, t4781: f64, t4939: f64, t1020: f64, t4801: f64, t92701: f64, t1092: f64, t14629: f64, t27763: f64, t3245: f64, t8054: f64, t1774: f64, t303: f64, t3170: f64) -> (f64, f64, f64, f64, f64) {
    let t95985 = t4939 * t4781 * t2845;
    let t95989 = t1020 * t92701 * t4801;
    let t95992 = t1092 * t27763 * t14629;
    let t96000 = t3245 * t8054;
    let t96003 = t303 * t3170 * t1774;
    (t95985, t95989, t95992, t96000, t96003)
}
