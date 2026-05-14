//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 977/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk977<F: Float>(t30247: F, t545: F, t2028: F, t2097: F, t6918: F, t7296: F, t2027: F, t2103: F, t26309: F, t26361: F, t26363: F, t26365: F, t27837: F, t28826: F, t28838: F, t28846: F, t28853: F, t28858: F, t28895: F, t28897: F, t28903: F, t28909: F, t30071: F, t6919: F, t7295: F, t7511: F, t7917: F, t8095: F, t8104: F) -> (F, F, F, F, F) {
    let t30295 = t545 * t30247;
    let t30296 = t2028 * t30295;
    let t30308 = t2097 * t6918;
    let t30309 = t7296 * t30308;
    let t30312 = 0.17347256376410398924e1 * t27837 * t8095 + t26309 - 0.10975748638225852664e-1 * t28826 + 0.19514881078765566038e-1 * t28838 - 0.65854491829355115987e0 * t7511 * t6919 + 0.14456046980341999104e-1 * t28846 - 0.19514881078765566038e-1 * t28853 - 0.4336814094102599731e0 * t2027 * t30296 - 0.25702851531048074406e-1 * t28858 - t26361 + t26363 - 0.8673628188205199462e0 * t7917 * t8104 - t26365 - 0.14456046980341999104e-1 * t28895 + 0.25702851531048074406e-1 * t28897 - 0.4336814094102599731e0 * t30071 * t2103 + 0.10975748638225852664e-1 * t28903 + 0.14456046980341999104e-1 * t28909 + 0.8673628188205199462e0 * t7295 * t30309;
    (t30295, t30296, t30308, t30309, t30312)
}
