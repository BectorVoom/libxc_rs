//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1205/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1205<F: Float>(t7798: F, t805: F, t2516: F, t243: F, t2519: F, t23789: F, t23804: F, t24693: F, t24696: F, t24702: F, t24737: F, t24752: F, t24759: F, t24775: F, t24788: F, t24792: F, t24795: F, t24799: F, t24804: F, t24824: F, t24839: F, t24855: F, t24870: F, t2488: F, t2493: F, t2495: F, t2518: F, t252: F, t2520: F, t7741: F, t7754: F, t7759: F, t7760: F, t7794: F, t7801: F, t7802: F, t7810: F, t810: F, t818: F) -> F {
    let t24876 = t805 * t7798;
    let t24879 = t2516 * t2516;
    let t24881 = t243 / t24879;
    let t24882 = t2519 * t2519;
    let t24883 = F::new(1.0) / t24882;
    let t24887 = -F::new(24.0) * t7759 * t24737 * t818 - F::new(6.0) * t2493 * t24752 * t818 + F::new(0.96494049533612093922e2) * t2518 * t24752 * t2520 - F::new(12.0) * t24759 * t2495 + F::new(0.14035736153892489771e2) * t7810 * t7741 - F::new(0.3109e-1) * (t24775 + t24788) * t252 + t24693 + t24696 + t24702 + F::new(0.41015588084031179722e4) * t24792 * t7754 + F::new(0.91080982599109921211e5) * t24795 * t23789 * t23804 - F::new(0.77195239626889675138e3) * t24799 * t7760 - F::new(0.24829604254387158296e5) * t24804 * t24737 * t7801 + F::new(4.0) * t2488 * t7794 + F::new(1.0) * t810 * (t24824 + t24839 + t24855 + t24870) * t818 + F::new(0.82765347514623860983e4) * t24876 * t7802 + F::new(0.19965908856856833625e6) * t24881 * t24737 * t24883;
    t24887
}
