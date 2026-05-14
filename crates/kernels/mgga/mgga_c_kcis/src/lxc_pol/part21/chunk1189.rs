//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1189/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1189<F: Float>(t27042: F, t28093: F, t46978: F, t7788: F, t8094: F, t95826: F, t1259: F, t417: F, t15610: F, t26997: F, t26954: F, t28189: F, t15471: F, t26955: F, t26963: F, t27014: F, t28102: F, t28211: F, t5329: F, t7794: F, t8087: F, t8095: F, t92604: F, t92657: F, t92948: F, t93028: F, t95828: F) -> (F, F, F) {
    let t96899 = t27042 * t28093;
    let t96902 = t7788 * t46978 * t8094;
    let t96904 = 0.15476481481481481481e-2 * t95826;
    let t96908 = t417 * t1259;
    let t96910 = t96908 * t15610 * t26997;
    let t96917 = t28189 * t26954;
    let t96920 = 0.69505208333333333334e-3 * t27014 * t28211 + 0.34752604166666666667e-3 * t7788 * t5329 * t7794 * t15471 + 0.45346742476851851853e-3 * t92948 * t8087 - 0.82448622685185185185e-4 * t96899 - 0.7722800925925925926e-4 * t96902 + t96904 - 0.38691203703703703704e-2 * t95828 - 0.18534722222222222222e-2 * t92604 * t8095 + 0.2782641015625e-3 * t26955 * t96910 + 0.185671721767578125e-4 * t92657 * t96910 + 0.30918233506944444444e-4 * t93028 * t28102 + 0.23168402777777777778e-3 * t96917 * t26963;
    (t96910, t96917, t96920)
}
