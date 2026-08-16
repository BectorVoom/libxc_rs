//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1173/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1173(t100533: f64, t3268: f64, t8513: f64, t31883: f64, t33822: f64, t1668: f64, t3143: f64, t100743: f64, t120335: f64, t120481: f64, t120532: f64, t120602: f64, t120636: f64, t120676: f64, t27606: f64, t27676: f64, t27680: f64, t3153: f64, t31891: f64, t31892: f64, t31897: f64, t31937: f64, t31940: f64, t31959: f64, t32022: f64, t33796: f64, t4983: f64, t7135: f64, t7160: f64, t7817: f64, t7828: f64, t988: f64, t999: f64) -> f64 {
    let t127053 = t8513 * t100533 * t3268;
    let t127059 = t33822 * t31883;
    let t127061 = t3143 * t1668;
    let t127074 = -0.34694512752820797848e1_f64 * t120335 * t27680 + 0.3427184259906141157e1_f64 * t31897 * t31892 * t7817 * t7135 - 0.3427184259906141157e1_f64 * t31891 * t31959 * t7828 * t7135 - 0.17347256376410398924e1_f64 * t31940 * t27676 + 0.99166211224136028853e-3_f64 * t33822 * t32022 - 0.17347256376410398924e1_f64 * t120676 * t27606 + 0.11423947533020470523e1_f64 * t127053 * t31937 + 0.34694512752820797848e1_f64 * t120532 * t7160 * t100743 - 0.12395776403017003607e-3_f64 * t127059 - 0.17347256376410398924e1_f64 * t120636 * t127061 * t3153 * t4983 - 0.51407763898592117355e1_f64 * t120602 * t31892 * t33796 * t988 + 0.6854368519812282314e1_f64 * t120481 * t31892 * t33796 * t999;
    t127074
}
