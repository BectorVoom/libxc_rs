//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1168/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1168<F: Float>(t35149: F, t604: F, t609: F, t1882: F, t35211: F, t1060: F, t12680: F, t13220: F, t1359: F, t139702: F, t140275: F, t140278: F, t140288: F, t140290: F, t144: F, t148451: F, t148613: F, t148860: F, t148880: F, t148897: F, t167: F, t1901: F, t2142: F, t2185: F, t26590: F, t26897: F, t27414: F, t32951: F, t33056: F, t3424: F, t3429: F, t34822: F, t35160: F, t446: F, t574: F, t5869: F, t5935: F, t616: F, t9144: F) -> (F, F) {
    let t148905 = t35149 * t604;
    let t148906 = t148905 * t609;
    let t148914 = t1882 * t35211;
    let t148921 = F::new(2.0) / F::new(3.0) * t446 * t574 * t5935 * t26897 - t446 * t144 * t148860 / F::new(3.0) - t1901 * t9144 * t139702 * t3424 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t1901 * t13220 * t139702 * t3429 + F::new(4.0) / F::new(3.0) * t446 * t2185 * t616 * t34822 + F::new(4.0) / F::new(3.0) * t446 * t2185 * t167 * t148451 + F::new(2.0) / F::new(3.0) * t446 * t144 * t148880 + F::new(2.0) / F::new(3.0) * t446 * t574 * t2142 * t35160 - F::new(2.0) / F::new(27.0) * t140275 + F::new(2.0) / F::new(3.0) * t446 * t2185 * t1060 * t32951 + F::new(2.0) / F::new(3.0) * t446 * t2185 * t167 * t148613 - F::new(2.0) / F::new(3.0) * t446 * t144 * t148897 - F::new(2.0) / F::new(3.0) * t446 * t574 * t27414 * t1359 - t446 * t144 * t148906 / F::new(3.0) + t140278 + F::new(2.0) / F::new(3.0) * t446 * t574 * t26590 * t5869 + F::new(2.0) / F::new(9.0) * t148914 - F::new(4.0) / F::new(9.0) * t140288 + F::new(2.0) / F::new(9.0) * t140290 - F::new(2.0) / F::new(9.0) * t1901 * t12680 * t33056;
    (t148906, t148921)
}
