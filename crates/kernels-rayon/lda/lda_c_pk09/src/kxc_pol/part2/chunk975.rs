//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 975/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk975(t10036: f64, t10044: f64, t10067: f64, t10138: f64, t10140: f64, t10143: f64, t10146: f64, t10151: f64, t10155: f64, t10177: f64, t10222: f64, t10223: f64, t10227: f64, t10255: f64, t10301: f64, t10333: f64, t10341: f64, t10401: f64, t1451: f64, t1611: f64, t1629: f64, t2571: f64, t2580: f64, t311: f64, t5043: f64, t5056: f64, t5087: f64, t5707: f64, t5710: f64, t5712: f64, t5773: f64, t5775: f64, t5830: f64, t5836: f64, t5840: f64, t5847: f64, t5854: f64, t5856: f64, t5939: f64, t5941: f64, t5982: f64, t5984: f64, t5986: f64, t6018: f64, t6020: f64, t6023: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9821: f64, t9823: f64) -> f64 {
    let t10405 = t10222 - t6018 / 18.0_f64 - t6020 / 18.0_f64 + t10036 + t10255 + t10301 + t10067 + t10140 / 18.0_f64 - 0.14975624337724558_f64 * t9821 - 0.14975624337724558_f64 * t9823 - 0.10237773105191754_f64 * t9746 - 0.20475546210383508_f64 * t9628 + t10155 / 6.0_f64 - t5830 / 6.0_f64 - t5982 + t5984 + t10143 / 6.0_f64 + t10146 / 6.0_f64 + t10138 + t10401 + 0.10237773105191754_f64 * t5043 + t10177 + t5707 + t5712 / 6.0_f64 - t5847 + t5856 / 18.0_f64 + t5710 - t10151 * t1451 / 6.0_f64 + t2571 * t1629 / 6.0_f64 - t10044 * t311 / 6.0_f64 + t10341 * t311 / 6.0_f64 + t10333 * t311 / 6.0_f64 - t10223 * t311 / 6.0_f64 + t10227 * t1611 / 12.0_f64 + t2580 * t1629 / 6.0_f64 - t6023 / 36.0_f64 - 0.10237773105191754_f64 * t9756 + t5840 - t5854 / 18.0_f64 - 0.03412591035063918_f64 * t9753 - t5986 + 0.03412591035063918_f64 * t5056 + t5939 / 18.0_f64 + t5941 / 18.0_f64 + t5773 / 18.0_f64 + t5775 / 18.0_f64 - 0.04991874779241519_f64 * t5087 + t5836;
    t10405
}
