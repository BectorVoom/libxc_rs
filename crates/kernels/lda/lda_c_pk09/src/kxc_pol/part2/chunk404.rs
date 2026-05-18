//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 404/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk404<F: Float>(t1672: F, t538: F, t132: F, t242: F, t142: F, t550: F, t546: F, t1684: F, t1735: F, t1666: F, t1669: F, t1905: F, t309: F, t633: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2019 = F::new(0.7380249726277691) * t538 * t1672;
    let t2020 = t242 * t132;
    let t2021 = t142 * t2020;
    let t2023 = F::new(2.3693919160612835) * t550 * t2021;
    let t2025 = F::new(0.8091720650647759) * t546 * t1672;
    let t2026 = F::new(0.10237773105191754) * t1684;
    let t2027 = F::new(0.03412591035063918) * t1735;
    let t2029 = F::new(0.04991874779241519) * t1666;
    let t2030 = F::new(0.01233429741534199) * t1669;
    let t2032 = t309 * t1905 * t633;
    (t2019, t2021, t2023, t2025, t2026, t2027, t2029, t2030, t2032)
}
