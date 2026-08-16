//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1124/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1124<F: Float>(t6708: F, t95021: F, t3408: F, t7312: F, t1882: F, t35073: F, t1384: F, t2179: F, t27191: F, t34947: F, t604: F, t1053: F, t106623: F, t12680: F, t13153: F, t1391: F, t140169: F, t144: F, t1901: F, t2142: F, t2210: F, t26768: F, t27329: F, t32869: F, t33035: F, t33040: F, t35125: F, t35229: F, t379: F, t446: F, t569: F, t574: F, t605: F) -> (F, F, F, F) {
    let t148120 = t95021 * t6708;
    let t148132 = t7312 * t3408;
    let t148163 = t1882 * t35073;
    let t148166 = t2179 * t1384 * t27191;
    let t148170 = t604 * t34947;
    let t148178 = t446 * t574 * t2142 * t35125 / F::cast_from(3.0_f64) + t446 * t574 * t605 * t32869 * t1053 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t574 * t1391 * t26768 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t140169 + t1901 * t12680 * t33035 / F::cast_from(9.0_f64) + t1901 * t13153 * t33040 / F::cast_from(9.0_f64) - t446 * t569 * t35229 * t379 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t148163 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t144 * t148166 + t1901 * t2210 * t148170 * t379 / F::cast_from(9.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t106623 * t27329;
    (t148120, t148132, t148166, t148178)
}
