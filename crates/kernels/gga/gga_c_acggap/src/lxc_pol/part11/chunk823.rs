//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 823/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk823<F: Float>(t7738: F, t7740: F, t7743: F, t7748: F, t7754: F, t7756: F, t7758: F, t7760: F, t8904: F, t8909: F, t8913: F, t8917: F, t8921: F, t8925: F, t8930: F) -> F {
    let t8936 = F::cast_from(0.53592522647587171215e-3_f64) * t8904 + F::cast_from(0.21437009059034868486e-3_f64) * t8909 - F::cast_from(0.10718504529517434243e-3_f64) * t8913 - F::new(0.4584375e-1) * t8917 - F::new(0.22921875e-1) * t8921 - F::new(0.22921875e-1) * t8925 - F::new(0.22921875e-1) * t8930 - t7738 - t7740 + t7743 + t7748 - F::cast_from(0.14291339372689912324e-3_f64) * t7754 + F::cast_from(0.32155513588552302729e-2_f64) * t7756 + F::cast_from(0.12862205435420921092e-2_f64) * t7758 - F::cast_from(0.53592522647587171215e-3_f64) * t7760;
    t8936
}
