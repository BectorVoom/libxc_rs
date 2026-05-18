//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1119/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1119<F: Float>(t35064: F, t8392: F, t160: F, t34918: F, t1882: F, t35060: F, t35127: F, t35091: F, t2178: F, t7339: F, t107082: F, t11593: F, t12968: F, t13140: F, t1378: F, t139757: F, t139767: F, t139791: F, t139950: F, t1901: F, t2210: F, t2221: F, t23455: F, t23571: F, t26520: F, t26849: F, t26897: F, t26924: F, t27015: F, t27333: F, t27336: F, t3052: F, t33034: F, t3455: F, t3478: F, t3483: F, t35094: F, t379: F, t446: F, t574: F, t5968: F, t605: F, t6615: F, t6626: F, t9099: F, t95696: F) -> F {
    let t147797 = t8392 * t35064;
    let t147806 = t160 * t34918;
    let t147830 = t1882 * t35060;
    let t147837 = t1882 * t35127;
    let t147839 = t8392 * t35091;
    let t147845 = t2178 * t7339;
    let t147855 = F::new(2.0) / F::new(9.0) * t1901 * t95696 * t6626 - F::new(4.0) * t1901 * t27333 * t1378 * t27336 - t147797 / F::new(27.0) - F::new(2.0) / F::new(9.0) * t1901 * t9099 * t35094 - F::new(4.0) / F::new(3.0) * t1901 * t13140 * t23455 * t26849 + t1901 * t2221 * t147806 * t379 / F::new(9.0) - F::new(4.0) / F::new(3.0) * t1901 * t107082 * t26924 - F::new(4.0) / F::new(3.0) * t1901 * t12968 * t23571 * t26897 - F::new(4.0) / F::new(3.0) * t1901 * t13140 * t27015 * t26520 + F::new(2.0) / F::new(9.0) * t11593 * t2210 * t33034 * t3052 + F::new(2.0) * t1901 * t13140 * t139757 * t3455 - t139767 - F::new(2.0) / F::new(9.0) * t147830 + F::new(2.0) / F::new(3.0) * t446 * t574 * t605 * t6615 * t5968 - t147837 / F::new(9.0) - t147839 / F::new(27.0) - F::new(2.0) / F::new(3.0) * t1901 * t12968 * t139950 * t3478 - F::new(2.0) / F::new(3.0) * t1901 * t13140 * t147845 * t3483 - F::new(2.0) / F::new(3.0) * t1901 * t12968 * t139950 * t3455 + F::new(2.0) / F::new(9.0) * t139791;
    t147855
}
