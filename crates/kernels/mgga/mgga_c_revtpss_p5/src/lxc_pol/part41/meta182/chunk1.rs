//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 753/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk753<F: Float>(t1063: F, t1671: F, t3082: F, t3086: F, t3091: F, t3169: F, t375: F, t4783: F, t4788: F, t4792: F, t4794: F, t4798: F, t4803: F, t4808: F, t4848: F, t4883: F, t4928: F) -> F {
    let t4930 = F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t4783 + F::cast_from(0.14291339372689912324e-3_f64) * t3091 * t4788 - t3082 - t3086 / F::cast_from(108.0_f64) + F::cast_from(0.14291339372689912324e-3_f64) * t4792 - F::cast_from(0.11433071498151929859e-2_f64) * t4794 * t375 + F::cast_from(0.21437009059034868486e-3_f64) * t4798 * t375 - F::cast_from(0.28582678745379824648e-3_f64) * t1063 * t4803 + F::cast_from(0.23818898954483187207e-3_f64) * t1063 * t4808 - F::cast_from(0.11433071498151929859e-2_f64) * t3169 * t1671 + t4848 + t4883 + t4928;
    t4930
}
