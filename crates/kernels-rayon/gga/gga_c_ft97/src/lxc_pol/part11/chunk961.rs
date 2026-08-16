//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 961/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk961(t120: f64, t12488: f64, t1554: f64, t1655: f64, t2007: f64, t2014: f64, t2015: f64, t2016: f64, t37627: f64, t37640: f64, t378: f64, t383: f64, t39533: f64, t39535: f64, t39539: f64, t39918: f64, t39922: f64, t39926: f64, t39932: f64, t39942: f64, t39976: f64, t422: f64, t528: f64, t72: f64, t7899: f64, t7977: f64, t8948: f64, t8950: f64, t8956: f64, t8959: f64, t8963: f64, t8964: f64, t8966: f64, t8967: f64, t8972: f64) -> f64 {
    let t39985 = 0.43406294696984965172e-2_f64 * t8963 * t39918 * t12488 - 0.59031789687271907074e-3_f64 * t39922 * t8967 + 0.22136921132726965153e-3_f64 * t39926 * t1554 * t7899 * t8966 + 0.17709536906181572122e-2_f64 * t8963 * t8964 * t39932 + 0.19923229019454268637e-2_f64 * t8948 * t378 * t2015 * t1655 - 0.44273842265453930305e-2_f64 * t8959 * t8956 + 0.22136921132726965153e-3_f64 * t39942 * t8950 - 0.79692916077817074549e-2_f64 * t2014 * t72 * t37627 * t120 - 0.90429780618718677442e-4_f64 * t8948 * t378 * t37640 * t528 * t120 - 0.10625722143708943273e-1_f64 * t2014 * t72 * t7977 * t383 * t120 - 0.36171912247487470976e-3_f64 * t2014 * t72 * t37640 * t2007 * t120 + 0.48229216329983294636e-3_f64 * t8959 * t8972 + 0.48229216329983294636e-3_f64 * t8963 * t422 * t7899 * t528 * t8966 - 0.44273842265453930305e-2_f64 * t8963 * t422 * t1655 * t383 * t8966 + 0.59031789687271907073e-3_f64 * t39976 * t2016 + 0.23410285231011484e0_f64 * t39535 * t120 - 0.438942848081465325e0_f64 * t39539 * t120 - 0.35115427846517226e0_f64 * t39533 * t120;
    t39985
}
