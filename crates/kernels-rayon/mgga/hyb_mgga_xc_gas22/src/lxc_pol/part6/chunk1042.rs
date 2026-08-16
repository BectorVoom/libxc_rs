//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1042/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1042(t2889: f64, t535: f64, t513: f64, t1134: f64, t1536: f64, t1543: f64, t1549: f64, t1556: f64, t2829: f64, t2838: f64, t2869: f64, t2876: f64, t3661: f64, t3688: f64, t3733: f64, t510: f64, t7602: f64, t7817: f64, t9485: f64, t9490: f64, t9598: f64, t9604: f64, t9715: f64, t9718: f64, t9737: f64, t9739: f64, t9742: f64, t9747: f64, t9750: f64, t9757: f64) -> (f64, f64, f64) {
    let t9761 = t535 * t2889;
    let t9762 = t9761 * t513;
    let t9764 = -88.0_f64 / 27.0_f64 * t2829 * t9485 + 400.0_f64 / 27.0_f64 * t3661 * t9490 + 400.0_f64 / 27.0_f64 * t3733 * t9490 + 64.0_f64 / 27.0_f64 * t3688 * t9715 + 32.0_f64 / 9.0_f64 * t2838 * t9718 - 8.0_f64 / 9.0_f64 * t7602 * t1556 + 252.0_f64 * t9737 * t9739 + 12.0_f64 * t9742 * t9739 + 400.0_f64 / 9.0_f64 * t9598 * t9604 + 2.0_f64 * t9747 * t1536 - 24.0_f64 * t510 * t9750 * t2876 + 120.0_f64 * t7817 * t1543 * t2869 + 252.0_f64 * t1134 * t9757 * t2876 + t9762 * t1549;
    (t9761, t9762, t9764)
}
