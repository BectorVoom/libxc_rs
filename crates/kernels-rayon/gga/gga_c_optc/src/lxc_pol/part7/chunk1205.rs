//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1205/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1205(t7798: f64, t805: f64, t2516: f64, t243: f64, t2519: f64, t23789: f64, t23804: f64, t24693: f64, t24696: f64, t24702: f64, t24737: f64, t24752: f64, t24759: f64, t24775: f64, t24788: f64, t24792: f64, t24795: f64, t24799: f64, t24804: f64, t24824: f64, t24839: f64, t24855: f64, t24870: f64, t2488: f64, t2493: f64, t2495: f64, t2518: f64, t252: f64, t2520: f64, t7741: f64, t7754: f64, t7759: f64, t7760: f64, t7794: f64, t7801: f64, t7802: f64, t7810: f64, t810: f64, t818: f64) -> f64 {
    let t24876 = t805 * t7798;
    let t24879 = t2516 * t2516;
    let t24881 = t243 / t24879;
    let t24882 = t2519 * t2519;
    let t24883 = 1.0_f64 / t24882;
    let t24887 = -24.0_f64 * t7759 * t24737 * t818 - 6.0_f64 * t2493 * t24752 * t818 + 0.96494049533612093922e2_f64 * t2518 * t24752 * t2520 - 12.0_f64 * t24759 * t2495 + 0.14035736153892489771e2_f64 * t7810 * t7741 - 0.3109e-1_f64 * (t24775 + t24788) * t252 + t24693 + t24696 + t24702 + 0.41015588084031179722e4_f64 * t24792 * t7754 + 0.91080982599109921211e5_f64 * t24795 * t23789 * t23804 - 0.77195239626889675138e3_f64 * t24799 * t7760 - 0.24829604254387158296e5_f64 * t24804 * t24737 * t7801 + 4.0_f64 * t2488 * t7794 + 1.0_f64 * t810 * (t24824 + t24839 + t24855 + t24870) * t818 + 0.82765347514623860983e4_f64 * t24876 * t7802 + 0.19965908856856833625e6_f64 * t24881 * t24737 * t24883;
    t24887
}
