//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1326/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1326<F: Float>(t39838: F, t39853: F, t162: F, t187: F, t10428: F, t2615: F, t2622: F, t9586: F, t2514: F, t2492: F) -> (F, F, F, F, F, F) {
    let t39854 = t39838 + t39853;
    let t39857 = F::cast_from(0.19751673498613801407e-1_f64) * t39854 * t162 * t187;
    let t39858 = t10428 * t2615;
    let t39859 = F::cast_from(48.0_f64) * t39858;
    let t39860 = t2622 * t9586;
    let t39861 = F::cast_from(0.22787578869697033845e-2_f64) * t39860;
    let t39871 = t2514 * t2514;
    let t39875 = t2492 * t2492;
    (t39854, t39857, t39859, t39861, t39871, t39875)
}
