//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 997/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk997<F: Float>(t10020: F, t1315: F, t10025: F, t10736: F, t10741: F, t10744: F, t10748: F, t10751: F, t10753: F, t10761: F, t1280: F, t1292: F, t1307: F, t1342: F, t1351: F, t1365: F, t2587: F, t311: F, t5547: F, t5550: F, t5570: F, t5572: F, t5576: F, t5579: F) -> F {
    let t10767 = t1315 * t10020;
    let t10771 = F::new(0.04115066352984959) * t10736 * t1351 - F::new(0.04115066352984959) * t10736 * t1365 - F::new(4.937333717448355) * t10741 * t1292 + F::new(4.937333717448355) * t10744 * t311 + F::new(0.04115066352984959) * t10748 - F::new(4.937333717448355) * t10751 - F::new(18.635258017632964) * t10753 * t1292 - F::new(2.2140749178833072) * t5547 + F::new(2.2140749178833072) * t5550 + F::new(0.9941357652469939) * t5570 + F::new(18.635258017632964) * t5572 - F::new(0.04115066352984959) * t5576 + F::new(1.8805371096875316) * t10761 + F::new(3.7610742193750633) * t1342 * t10025 - F::new(1.8805371096875316) * t1280 * t2587 - F::new(3.7610742193750633) * t10767 - t5579 - F::new(7.5221484387501265) * t1307 * t10025;
    t10771
}
