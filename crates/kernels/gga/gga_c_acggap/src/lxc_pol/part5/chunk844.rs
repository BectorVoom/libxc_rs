//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 844/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk844<F: Float>(t11795: F, t11797: F, t11800: F, t11803: F, t11806: F, t11811: F, t11813: F, t11815: F, t11817: F, t11820: F, t201: F, t219: F) -> F {
    let t11825 = F::new(1.0) * t201 * (-F::cast_from(0.21099166666666666667e1_f64) * t11795 + F::new(0.202552e2) * t11797 - F::cast_from(0.75019259259259259258e1_f64) * t11800 + F::cast_from(0.6564185185185185185e1_f64) * t11803 + F::cast_from(0.31003950617283950618e1_f64) * t11806 + F::cast_from(0.68258333333333333335e-1_f64) * t11811 - F::cast_from(0.10921333333333333333e1_f64) * t11813 + F::cast_from(0.12134814814814814815e1_f64) * t11815 + F::cast_from(0.10617962962962962963e1_f64) * t11817 + F::cast_from(0.13388493827160493828e1_f64) * t11820) * t219;
    t11825
}
