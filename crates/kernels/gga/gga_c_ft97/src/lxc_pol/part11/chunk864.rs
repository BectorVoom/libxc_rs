//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 864/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk864<F: Float>(t1655: F, t71: F, t2264: F, t341: F, t17: F, t8946: F, t8947: F, t120: F, t1570: F, t1559: F, t16: F, t2252: F, t12488: F, t1554: F, t2007: F, t2014: F, t2015: F, t2016: F, t37627: F, t37640: F, t378: F, t383: F, t39533: F, t39535: F, t39539: F, t422: F, t528: F, t72: F, t7899: F, t7977: F, t8948: F, t8950: F, t8956: F, t8959: F, t8963: F, t8964: F, t8966: F, t8967: F, t8972: F) -> (F, F, F, F, F) {
    let t39918 = t71 * t1655;
    let t39922 = t341 * t2264;
    let t39926 = t8946 * t8947 * t17;
    let t39931 = t120 * t1570;
    let t39932 = t39931 * t1559;
    let t39942 = t8946 * t16;
    let t39976 = t341 * t2252;
    let t39985 = 0.43406294696984965172e-2 * t8963 * t39918 * t12488 - 0.59031789687271907074e-3 * t39922 * t8967 + 0.22136921132726965153e-3 * t39926 * t1554 * t7899 * t8966 + 0.17709536906181572122e-2 * t8963 * t8964 * t39932 + 0.19923229019454268637e-2 * t8948 * t378 * t2015 * t1655 - 0.44273842265453930305e-2 * t8959 * t8956 + 0.22136921132726965153e-3 * t39942 * t8950 - 0.79692916077817074549e-2 * t2014 * t72 * t37627 * t120 - 0.90429780618718677442e-4 * t8948 * t378 * t37640 * t528 * t120 - 0.10625722143708943273e-1 * t2014 * t72 * t7977 * t383 * t120 - 0.36171912247487470976e-3 * t2014 * t72 * t37640 * t2007 * t120 + 0.48229216329983294636e-3 * t8959 * t8972 + 0.48229216329983294636e-3 * t8963 * t422 * t7899 * t528 * t8966 - 0.44273842265453930305e-2 * t8963 * t422 * t1655 * t383 * t8966 + 0.59031789687271907073e-3 * t39976 * t2016 + 0.23410285231011484e0 * t39535 * t120 - 0.438942848081465325e0 * t39539 * t120 - 0.35115427846517226e0 * t39533 * t120;
    (t39922, t39926, t39942, t39976, t39985)
}
