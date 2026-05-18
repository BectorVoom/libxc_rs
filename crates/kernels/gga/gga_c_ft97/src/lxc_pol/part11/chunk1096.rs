//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1096/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1096<F: Float>(t10151: F, t1882: F, t259: F, t41743: F, t89: F, t754: F, t9802: F, t10007: F, t10014: F, t10018: F, t10051: F, t10053: F, t10092: F, t13879: F, t14098: F, t1901: F, t2373: F, t2409: F, t2574: F, t2606: F, t2619: F, t3891: F, t42404: F, t42417: F, t446: F, t684: F, t9787: F, t9804: F, t9808: F, t9849: F) -> F {
    let t42920 = t1882 * t10151;
    let t42928 = F::new(280.0) / F::new(243.0) * t89 * t41743 * t259;
    let t42939 = t9802 * t754;
    let t42960 = F::new(4.0) / F::new(3.0) * t42920 - F::new(4.0) / F::new(3.0) * t1901 * t10007 * t10018 * t684 + t42928 + F::new(8.0) / F::new(3.0) * t1901 * t3891 * t14098 * t42404 - F::new(8.0) / F::new(9.0) * t1901 * t13879 * t10014 + F::new(4.0) / F::new(3.0) * t1901 * t9787 * t9849 + F::new(8.0) / F::new(9.0) * t1901 * t42939 * t9804 + F::new(8.0) / F::new(3.0) * t1901 * t2606 * t10051 * t10053 * t684 + F::new(8.0) / F::new(3.0) * t1901 * t2606 * t9808 * t42417 - F::new(4.0) / F::new(3.0) * t1901 * t2606 * t10092 * t2409 + F::new(4.0) * t446 * t2574 * t2619 * t2373;
    t42960
}
