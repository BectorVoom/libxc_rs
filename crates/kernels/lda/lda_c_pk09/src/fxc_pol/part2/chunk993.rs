//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 993/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk993<F: Float>(t10686: F, t623: F, t93: F, t1215: F, t2636: F, t10025: F, t10623: F, t10626: F, t10660: F, t10669: F, t10679: F, t10684: F, t1292: F, t1307: F, t1476: F, t1490: F, t1609: F, t2513: F, t2521: F, t2641: F, t311: F, t5108: F, t5328: F, t5455: F, t5464: F, t5477: F, t5480: F, t5613: F, t9770: F) -> F {
    let t10687 = t10686 * t623;
    let t10688 = t93 * t10687;
    let t10692 = t2636 * t1215;
    let t10697 = F::new(4.937333717448355) * t10623 * t311 + F::new(19.489173774580152) * t10626 * t311 + F::new(1.8805371096875316) * t10660 * t311 + F::new(3.7610742193750633) * t5108 * t2513 + F::new(3.7610742193750633) * t1307 * t9770 + F::new(1.8805371096875316) * t1609 * t2521 + F::new(2.427516195194328) * t10669 * t1292 + F::new(2.427516195194328) * t5464 * t2513 + F::new(2.427516195194328) * t1490 * t9770 - F::new(4.855032390388656) * t1490 * t10025 - F::new(3.5540878740919255) * t5613 * t10679 + F::new(14.216351496367702) * t10684 + F::new(14.216351496367702) * t1476 * t10688 - F::new(0.8091720650647759) * t5455 + F::new(4.937333717448355) * t10692 * t311 + F::new(0.04115066352984959) * t5328 * t2641 - t5477 - t5480;
    t10697
}
