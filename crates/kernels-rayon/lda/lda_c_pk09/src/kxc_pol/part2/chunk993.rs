//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 993/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk993(t10686: f64, t623: f64, t93: f64, t1215: f64, t2636: f64, t10025: f64, t10623: f64, t10626: f64, t10660: f64, t10669: f64, t10679: f64, t10684: f64, t1292: f64, t1307: f64, t1476: f64, t1490: f64, t1609: f64, t2513: f64, t2521: f64, t2641: f64, t311: f64, t5108: f64, t5328: f64, t5455: f64, t5464: f64, t5477: f64, t5480: f64, t5613: f64, t9770: f64) -> f64 {
    let t10687 = t10686 * t623;
    let t10688 = t93 * t10687;
    let t10692 = t2636 * t1215;
    let t10697 = 4.937333717448355_f64 * t10623 * t311 + 19.489173774580152_f64 * t10626 * t311 + 1.8805371096875316_f64 * t10660 * t311 + 3.7610742193750633_f64 * t5108 * t2513 + 3.7610742193750633_f64 * t1307 * t9770 + 1.8805371096875316_f64 * t1609 * t2521 + 2.427516195194328_f64 * t10669 * t1292 + 2.427516195194328_f64 * t5464 * t2513 + 2.427516195194328_f64 * t1490 * t9770 - 4.855032390388656_f64 * t1490 * t10025 - 3.5540878740919255_f64 * t5613 * t10679 + 14.216351496367702_f64 * t10684 + 14.216351496367702_f64 * t1476 * t10688 - 0.8091720650647759_f64 * t5455 + 4.937333717448355_f64 * t10692 * t311 + 0.04115066352984959_f64 * t5328 * t2641 - t5477 - t5480;
    t10697
}
