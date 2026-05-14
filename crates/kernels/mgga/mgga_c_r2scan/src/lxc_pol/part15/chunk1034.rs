//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1034/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1034<F: Float>(t39857: F, t10722: F, t8240: F, t11705: F, t6465: F, t2184: F, t24031: F, t3308: F, t1592: F, t24035: F, t37925: F, t37933: F, t39838: F, t39843: F, t39846: F, t39851: F, t39855: F) -> (F,) {
    let t39858 = 0.27738309568173659402e1 * t39857;
    let t39859 = t8240 * t10722;
    let t39863 = t6465 * t11705;
    let t39866 = t2184 * t3308 * t24031;
    let t39869 = t1592 * t3308 * t24035;
    let t39871 = 0.43663693315433241792e-2 * t39838 - 0.13099107994629972538e-1 * t39843 - 0.42377972951376424087e0 * t39846 - 0.65854491829355115988e0 * t39851 - t39855 - t39858 + 0.2600466522016280569e0 * t39859 + 0.64025200389650807209e-1 * t37925 - 0.42683466926433871472e0 * t37933 + 0.17336443480108537126e0 * t39863 + 0.17336443480108537126e0 * t39866 + 0.2600466522016280569e0 * t39869;
    (t39871,)
}
