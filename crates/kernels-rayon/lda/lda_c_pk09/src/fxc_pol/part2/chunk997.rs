//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 997/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk997(t10020: f64, t1315: f64, t10025: f64, t10736: f64, t10741: f64, t10744: f64, t10748: f64, t10751: f64, t10753: f64, t10761: f64, t1280: f64, t1292: f64, t1307: f64, t1342: f64, t1351: f64, t1365: f64, t2587: f64, t311: f64, t5547: f64, t5550: f64, t5570: f64, t5572: f64, t5576: f64, t5579: f64) -> f64 {
    let t10767 = t1315 * t10020;
    let t10771 = 0.04115066352984959_f64 * t10736 * t1351 - 0.04115066352984959_f64 * t10736 * t1365 - 4.937333717448355_f64 * t10741 * t1292 + 4.937333717448355_f64 * t10744 * t311 + 0.04115066352984959_f64 * t10748 - 4.937333717448355_f64 * t10751 - 18.635258017632964_f64 * t10753 * t1292 - 2.2140749178833072_f64 * t5547 + 2.2140749178833072_f64 * t5550 + 0.9941357652469939_f64 * t5570 + 18.635258017632964_f64 * t5572 - 0.04115066352984959_f64 * t5576 + 1.8805371096875316_f64 * t10761 + 3.7610742193750633_f64 * t1342 * t10025 - 1.8805371096875316_f64 * t1280 * t2587 - 3.7610742193750633_f64 * t10767 - t5579 - 7.5221484387501265_f64 * t1307 * t10025;
    t10771
}
