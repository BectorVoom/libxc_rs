//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1426/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1426(t5: f64, t83732: f64, t83766: f64, t83812: f64, t83849: f64, t112: f64, t531: f64, t6995: f64, t1983: f64, t22596: f64, t12012: f64, t1390: f64, t6878: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t83852 = piecewise3(t8, 0.0_f64, t83732 + t83766 + t83812 + t83849);
    let t83853 = t83852 * t112;
    let t83859 = t531 * t6995;
    let t83862 = 18.0_f64 * t1983 * t83859 * t22596;
    let t83863 = t1390 * t12012;
    let t83866 = 3.0_f64 * t1983 * t6878 * t83863;
    (t83853, t83862, t83866)
}
