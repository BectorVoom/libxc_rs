//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1350/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1350(t15471: f64, t26955: f64, t26963: f64, t27014: f64, t28102: f64, t28211: f64, t5329: f64, t7788: f64, t7794: f64, t8087: f64, t8095: f64, t92604: f64, t92657: f64, t92948: f64, t93028: f64, t95828: f64, t96899: f64, t96902: f64, t96904: f64, t96910: f64, t96917: f64) -> f64 {
    let t96920 = 0.69505208333333333334e-3_f64 * t27014 * t28211 + 0.34752604166666666667e-3_f64 * t7788 * t5329 * t7794 * t15471 + 0.45346742476851851853e-3_f64 * t92948 * t8087 - 0.82448622685185185185e-4_f64 * t96899 - 0.7722800925925925926e-4_f64 * t96902 + t96904 - 0.38691203703703703704e-2_f64 * t95828 - 0.18534722222222222222e-2_f64 * t92604 * t8095 + 0.2782641015625e-3_f64 * t26955 * t96910 + 0.185671721767578125e-4_f64 * t92657 * t96910 + 0.30918233506944444444e-4_f64 * t93028 * t28102 + 0.23168402777777777778e-3_f64 * t96917 * t26963;
    t96920
}
