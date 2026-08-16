//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1057/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1057(t2173: f64, t27856: f64, t1087: f64, t1774: f64, t303: f64, t26760: f64, t4801: f64, t1020: f64, t4806: f64, t7718: f64, t4548: f64, t4556: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27857 = t2173 * t27856;
    let t27859 = t1087 * t1774;
    let t27860 = t303 * t27859;
    let t27864 = t26760 * t4801;
    let t27865 = t1020 * t27864;
    let t27867 = t7718 * t4806;
    let t27868 = t1020 * t27867;
    let t27870 = t7718 * t4548;
    let t27871 = t1020 * t27870;
    let t27873 = t7718 * t4556;
    (t27857, t27859, t27860, t27864, t27865, t27867, t27868, t27870, t27871, t27873)
}
