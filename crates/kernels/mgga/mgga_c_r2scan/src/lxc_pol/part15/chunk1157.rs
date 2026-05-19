//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1157/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1157<F: Float>(t37925: F, t37933: F, t39838: F, t39843: F, t39846: F, t39851: F, t39855: F, t39858: F, t39859: F, t39863: F, t39866: F, t39869: F) -> F {
    let t39871 = F::cast_from(0.43663693315433241792e-2_f64) * t39838 - F::cast_from(0.13099107994629972538e-1_f64) * t39843 - F::cast_from(0.42377972951376424087e0_f64) * t39846 - F::cast_from(0.65854491829355115988e0_f64) * t39851 - t39855 - t39858 + F::cast_from(0.2600466522016280569e0_f64) * t39859 + F::cast_from(0.64025200389650807209e-1_f64) * t37925 - F::cast_from(0.42683466926433871472e0_f64) * t37933 + F::cast_from(0.17336443480108537126e0_f64) * t39863 + F::cast_from(0.17336443480108537126e0_f64) * t39866 + F::cast_from(0.2600466522016280569e0_f64) * t39869;
    t39871
}
