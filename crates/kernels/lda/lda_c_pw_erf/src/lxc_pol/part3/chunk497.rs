//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 497/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk497<F: Float>(t1313: F, t2030: F, t519: F, t549: F, t816: F, t1319: F, t1318: F, t1451: F, t1477: F, t1516: F, t1629: F, t1632: F, t1637: F, t1641: F, t1994: F, t1999: F, t2004: F, t2009: F, t2013: F, t2016: F, t2020: F, t2025: F, t2029: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2031 = t1313 * t2030;
    let t2033 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t519 * t2031;
    let t2034 = t816 * t549;
    let t2035 = t1319 * t2034;
    let t2037 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1318 * t2035;
    let t2039 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t1451;
    let t2040 = F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t1477;
    let t2041 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t1516;
    let t2042 = t1994 - t1999 + t2004 + t2009 - t2013 - t2016 + t2020 - t2025 + t2029 - t2033 + t2037 + t1629 + F::cast_from(0.10821041362364843_f64) * t1632 + t1637 + t1641 + t2039 + t2040 + t2041;
    (t2031, t2033, t2034, t2035, t2037, t2039, t2040, t2041, t2042)
}
