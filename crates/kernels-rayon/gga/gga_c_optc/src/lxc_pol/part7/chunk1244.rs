//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1244/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1244(t2586: f64, t8160: f64, t953: f64, t8140: f64, t8144: f64, t10918: f64, t11368: f64, t11369: f64, t11451: f64, t11455: f64, t23825: f64, t25419: f64, t25540: f64, t25633: f64, t25657: f64, t25660: f64, t25664: f64, t25667: f64, t25670: f64, t2668: f64, t2721: f64, t2797: f64, t3608: f64, t7397: f64, t7988: f64, t7992: f64, t8037: f64, t8149: f64, t8171: f64, t914: f64, t930: f64) -> f64 {
    let t25682 = t953 * t2586 * t8160;
    let t25684 = t8140 * t8144;
    let t25689 = 0.6058720680803250206e1_f64 * t11368 * t11369 * t25419 - 0.93770531639908660928e4_f64 * t11451 * t7988 - 0.16156588482142000549e2_f64 * t8149 * t8171 + 0.46885265819954330464e4_f64 * t11455 * t7992 + 0.15146801702008125515e1_f64 * t25657 + 0.15146801702008125515e1_f64 * t25660 + 0.11721316454988582616e4_f64 * t25664 + 0.20195735602677500687e1_f64 * t25667 - 0.58606582274942913081e3_f64 * t25670 + 0.10431793787746509425e1_f64 * t930 * t914 * t7397 * t23825 + 0.18545411178216016757e1_f64 * t2797 * t8037 - 0.12117441361606500412e2_f64 * t2721 * t3608 * t25633 + 0.6717427261115226305e-1_f64 * t25682 - 0.33268896651293990656e3_f64 * t25684 - 0.51620760404990155789e2_f64 * t2668 * t25540 * t10918;
    t25689
}
