//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1173/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1173<F: Float>(t100533: F, t3268: F, t8513: F, t31883: F, t33822: F, t1668: F, t3143: F, t100743: F, t120335: F, t120481: F, t120532: F, t120602: F, t120636: F, t120676: F, t27606: F, t27676: F, t27680: F, t3153: F, t31891: F, t31892: F, t31897: F, t31937: F, t31940: F, t31959: F, t32022: F, t33796: F, t4983: F, t7135: F, t7160: F, t7817: F, t7828: F, t988: F, t999: F) -> F {
    let t127053 = t8513 * t100533 * t3268;
    let t127059 = t33822 * t31883;
    let t127061 = t3143 * t1668;
    let t127074 = -F::cast_from(0.34694512752820797848e1_f64) * t120335 * t27680 + F::cast_from(0.3427184259906141157e1_f64) * t31897 * t31892 * t7817 * t7135 - F::cast_from(0.3427184259906141157e1_f64) * t31891 * t31959 * t7828 * t7135 - F::cast_from(0.17347256376410398924e1_f64) * t31940 * t27676 + F::cast_from(0.99166211224136028853e-3_f64) * t33822 * t32022 - F::cast_from(0.17347256376410398924e1_f64) * t120676 * t27606 + F::cast_from(0.11423947533020470523e1_f64) * t127053 * t31937 + F::cast_from(0.34694512752820797848e1_f64) * t120532 * t7160 * t100743 - F::cast_from(0.12395776403017003607e-3_f64) * t127059 - F::cast_from(0.17347256376410398924e1_f64) * t120636 * t127061 * t3153 * t4983 - F::cast_from(0.51407763898592117355e1_f64) * t120602 * t31892 * t33796 * t988 + F::cast_from(0.6854368519812282314e1_f64) * t120481 * t31892 * t33796 * t999;
    t127074
}
