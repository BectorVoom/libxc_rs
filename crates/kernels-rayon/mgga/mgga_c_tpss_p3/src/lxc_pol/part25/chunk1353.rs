//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1353/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1353(t21805: f64, t219: f64, t1265: f64, t1266: f64, t13850: f64, t1656: f64, t1838: f64, t18483: f64, t18490: f64, t18496: f64, t18950: f64, t19509: f64, t19542: f64, t19554: f64, t20171: f64, t20190: f64, t20196: f64, t20200: f64, t20211: f64, t21819: f64, t21826: f64, t21836: f64, t21841: f64, t21846: f64, t21849: f64, t520: f64, t538: f64, t5407: f64, t5449: f64, t5739: f64, t5745: f64, t5918: f64, t5930: f64, t60811: f64, t6430: f64, t65667: f64, t67061: f64, t69458: f64, t71748: f64, t71809: f64, param_beta: f64) -> f64 {
    let t71837 = t21805 * t219;
    let t71872 = 2.0_f64 * t18483 * t21841 + param_beta * t71809 * t538 + 2.0_f64 * t65667 * t6430 + 8.0_f64 * t18496 * t20190 * t1656 * t19542 - 4.0_f64 * t18496 * t67061 * t19554 - t71837 * t1266 + t5739 * t5745 * t5918 * t5407 * t520 + t5739 * t5745 * t1838 * t13850 * t520 + t5739 * t5745 * t71748 * t520 + t18483 * t21849 + 2.0_f64 * t19509 * t20196 + 2.0_f64 * t19509 * t20200 + t69458 * t5930 - 2.0_f64 * t18483 * t21836 - t18950 * t5449 + t18483 * t21846 + 2.0_f64 * t19509 * t20211 + 24.0_f64 * t5739 * t60811 * t21819 * t1265 - 12.0_f64 * t5739 * t18490 * t21826 * t1265 - 12.0_f64 * t19509 * t20171;
    t71872
}
