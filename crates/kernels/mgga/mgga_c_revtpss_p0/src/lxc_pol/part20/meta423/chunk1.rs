//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1590/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1590<F: Float>(t1145: F, t141: F, t43847: F, t12283: F, t698: F, t43858: F, t43862: F, t43865: F, t43871: F, t43877: F, t43883: F, t43909: F, t43911: F, t43914: F, t43917: F, t43920: F, t43923: F) -> (F, F, F) {
    let t43926 = t141 * t1145 * t43847;
    let t43928 = t698 * t12283;
    let t43936 = F::cast_from(0.258925e1_f64) * t43909 - F::cast_from(0.18396666666666666667e0_f64) * t43911 - F::cast_from(0.82785e-1_f64) * t43914 + F::cast_from(0.49671e0_f64) * t43917 - F::cast_from(0.11038e0_f64) * t43920 - F::cast_from(0.99342e0_f64) * t43923 + F::cast_from(0.66228e0_f64) * t43926 + F::cast_from(0.22076e0_f64) * t43928 - F::cast_from(0.44729629629629629629e0_f64) * t43858 - F::cast_from(0.89459259259259259259e0_f64) * t43862 - F::cast_from(0.53675555555555555556e0_f64) * t43865 - F::cast_from(0.60384999999999999999e0_f64) * t43871 + F::cast_from(0.181155e1_f64) * t43877 + F::cast_from(0.16102666666666666667e1_f64) * t43883;
    (t43926, t43928, t43936)
}
