//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 996/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk996(t2487: f64, t309: f64, t365: f64, t9602: f64, t1331: f64, t9819: f64, t1339: f64, t1318: f64, t1287: f64, t382: f64, t10020: f64, t1311: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10736 = t2487 * t309;
    let t10741 = t365 * t9602;
    let t10744 = t1331 * t2487;
    let t10747 = t9819 * t309;
    let t10748 = t10747 * t1339;
    let t10750 = t1318 * t9602;
    let t10751 = t10750 * t1287;
    let t10753 = t382 * t9602;
    let t10761 = t1311 * t10020;
    (t10736, t10741, t10744, t10748, t10751, t10753, t10761)
}
