//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1651/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1651<F: Float>(t43813: F, t43816: F, t43808: F, t43810: F, t43823: F, t43826: F, t43828: F, t43830: F, t43832: F, t43837: F, t43841: F, t43845: F, t43849: F, t43854: F) -> F {
    let t45106 = F::cast_from(0.5356037037037037037e1_f64) * t43813;
    let t45107 = F::cast_from(0.16979925925925925926e1_f64) * t43816;
    let t45118 = -F::new(0.94674375e0) * t43808 + F::new(0.1262325e1) * t43810 + t45106 + t45107 - F::cast_from(0.6618234375e1_f64) * t43823 - F::new(0.52945875e1) * t43826 - F::new(0.166712e1) * t43828 - F::new(0.41318e1) * t43830 + F::cast_from(0.13772666666666666667e1_f64) * t43832 + F::cast_from(0.34431666666666666667e1_f64) * t43837 - F::cast_from(0.13772666666666666667e1_f64) * t43841 + F::new(0.185931e2) * t43845 + F::new(0.41318e1) * t43849 - F::new(0.123954e2) * t43854;
    t45118
}
