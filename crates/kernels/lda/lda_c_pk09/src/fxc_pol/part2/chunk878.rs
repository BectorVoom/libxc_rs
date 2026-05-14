//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 878/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk878<F: Float>(t2487: F, t309: F, t365: F, t9602: F, t1331: F, t9819: F, t1339: F, t1318: F, t1287: F, t382: F, t10020: F, t1311: F, t1315: F, t10025: F, t1280: F, t1292: F, t1307: F, t1342: F, t1351: F, t1365: F, t2587: F, t311: F, t5547: F, t5550: F, t5570: F, t5572: F, t5576: F, t5579: F) -> (F,) {
    let t10736 = t2487 * t309;
    let t10741 = t365 * t9602;
    let t10744 = t1331 * t2487;
    let t10747 = t9819 * t309;
    let t10748 = t10747 * t1339;
    let t10750 = t1318 * t9602;
    let t10751 = t10750 * t1287;
    let t10753 = t382 * t9602;
    let t10761 = t1311 * t10020;
    let t10767 = t1315 * t10020;
    let t10771 = 0.04115066352984959 * t10736 * t1351 - 0.04115066352984959 * t10736 * t1365 - 4.937333717448355 * t10741 * t1292 + 4.937333717448355 * t10744 * t311 + 0.04115066352984959 * t10748 - 4.937333717448355 * t10751 - 18.635258017632964 * t10753 * t1292 - 2.2140749178833072 * t5547 + 2.2140749178833072 * t5550 + 0.9941357652469939 * t5570 + 18.635258017632964 * t5572 - 0.04115066352984959 * t5576 + 1.8805371096875316 * t10761 + 3.7610742193750633 * t1342 * t10025 - 1.8805371096875316 * t1280 * t2587 - 3.7610742193750633 * t10767 - t5579 - 7.5221484387501265 * t1307 * t10025;
    (t10771,)
}
