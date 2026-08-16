//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 995/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk995(t10719: f64, t1362: f64, t310: f64, t10020: f64, t1369: f64, t2649: f64, t623: f64, t1349: f64, t10025: f64, t10181: f64, t10249: f64, t10287: f64, t10703: f64, t10706: f64, t10713: f64, t10715: f64, t1322: f64, t1332: f64, t1345: f64, t1348: f64, t1629: f64, t2587: f64, t2637: f64, t374: f64, t4775: f64, t5482: f64, t5484: f64, t5511: f64, t5544: f64, t5546: f64) -> f64 {
    let t10720 = t10719 * t1362;
    let t10721 = t310 * t10720;
    let t10724 = t1369 * t10020;
    let t10728 = t2649 * t623;
    let t10729 = t1349 * t10728;
    let t10735 = -18.635258017632964_f64 * t5482 - 0.6268457032291772_f64 * t5484 + t5511 - 0.04115066352984959_f64 * t10181 * t374 + 2.0_f64 * t4775 * t10703 + 4.937333717448355_f64 * t10706 + 9.87466743489671_f64 * t1322 * t10025 - 4.937333717448355_f64 * t1332 * t2587 - 4.937333717448355_f64 * t10713 * t10715 + 5.40024514194619_f64 * t10249 + 0.04115066352984959_f64 * t1348 * t10721 + 18.635258017632964_f64 * t10724 + 37.27051603526593_f64 * t1345 * t10025 + 0.04115066352984959_f64 * t1348 * t10729 - 4.937333717448355_f64 * t2637 * t1629 - 7.35994946043302_f64 * t10287 - t5544 + t5546;
    t10735
}
