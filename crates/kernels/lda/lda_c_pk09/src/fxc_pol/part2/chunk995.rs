//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 995/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk995<F: Float>(t10719: F, t1362: F, t310: F, t10020: F, t1369: F, t2649: F, t623: F, t1349: F, t10025: F, t10181: F, t10249: F, t10287: F, t10703: F, t10706: F, t10713: F, t10715: F, t1322: F, t1332: F, t1345: F, t1348: F, t1629: F, t2587: F, t2637: F, t374: F, t4775: F, t5482: F, t5484: F, t5511: F, t5544: F, t5546: F) -> F {
    let t10720 = t10719 * t1362;
    let t10721 = t310 * t10720;
    let t10724 = t1369 * t10020;
    let t10728 = t2649 * t623;
    let t10729 = t1349 * t10728;
    let t10735 = -F::new(18.635258017632964) * t5482 - F::new(0.6268457032291772) * t5484 + t5511 - F::new(0.04115066352984959) * t10181 * t374 + F::new(2.0) * t4775 * t10703 + F::new(4.937333717448355) * t10706 + F::new(9.87466743489671) * t1322 * t10025 - F::new(4.937333717448355) * t1332 * t2587 - F::new(4.937333717448355) * t10713 * t10715 + F::new(5.40024514194619) * t10249 + F::new(0.04115066352984959) * t1348 * t10721 + F::new(18.635258017632964) * t10724 + F::new(37.27051603526593) * t1345 * t10025 + F::new(0.04115066352984959) * t1348 * t10729 - F::new(4.937333717448355) * t2637 * t1629 - F::new(7.35994946043302) * t10287 - t5544 + t5546;
    t10735
}
