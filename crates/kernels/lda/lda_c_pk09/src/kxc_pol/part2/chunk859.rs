//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 859/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk859<F: Float>(t10036: F, t10044: F, t10067: F, t10138: F, t10140: F, t10143: F, t10146: F, t10151: F, t10155: F, t10177: F, t10222: F, t10223: F, t10227: F, t10255: F, t10301: F, t10333: F, t10341: F, t10401: F, t1451: F, t1611: F, t1629: F, t2571: F, t2580: F, t311: F, t5043: F, t5056: F, t5087: F, t5707: F, t5710: F, t5712: F, t5773: F, t5775: F, t5830: F, t5836: F, t5840: F, t5847: F, t5854: F, t5856: F, t5939: F, t5941: F, t5982: F, t5984: F, t5986: F, t6018: F, t6020: F, t6023: F, t9628: F, t9746: F, t9753: F, t9756: F, t9821: F, t9823: F) -> (F,) {
    let t10405 = t10222 - t6018 / 18.0 - t6020 / 18.0 + t10036 + t10255 + t10301 + t10067 + t10140 / 18.0 - 0.14975624337724558 * t9821 - 0.14975624337724558 * t9823 - 0.10237773105191754 * t9746 - 0.20475546210383508 * t9628 + t10155 / 6.0 - t5830 / 6.0 - t5982 + t5984 + t10143 / 6.0 + t10146 / 6.0 + t10138 + t10401 + 0.10237773105191754 * t5043 + t10177 + t5707 + t5712 / 6.0 - t5847 + t5856 / 18.0 + t5710 - t10151 * t1451 / 6.0 + t2571 * t1629 / 6.0 - t10044 * t311 / 6.0 + t10341 * t311 / 6.0 + t10333 * t311 / 6.0 - t10223 * t311 / 6.0 + t10227 * t1611 / 12.0 + t2580 * t1629 / 6.0 - t6023 / 36.0 - 0.10237773105191754 * t9756 + t5840 - t5854 / 18.0 - 0.03412591035063918 * t9753 - t5986 + 0.03412591035063918 * t5056 + t5939 / 18.0 + t5941 / 18.0 + t5773 / 18.0 + t5775 / 18.0 - 0.04991874779241519 * t5087 + t5836;
    (t10405,)
}
