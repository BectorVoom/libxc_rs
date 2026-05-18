//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1327/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1327<F: Float>(t11153: F, t931: F, t11445: F, t154: F, t18994: F, t385: F, t2347: F, t3171: F, t3849: F, t10038: F, t11383: F, t1220: F, t19055: F, t19124: F, t19153: F, t19163: F, t23272: F, t2888: F, t31086: F, t3174: F, t3181: F, t824: F, t907: F, t909: F) -> F {
    let t32143 = t931 * t11153;
    let t32150 = t385 * t154 * t18994 * t11445;
    let t32164 = t385 * t154 * t2347 * t11153;
    let t32166 = t3849 * t3171;
    let t32168 = -t19055 + F::new(0.63517063878621832551e-4) * t19124 - F::new(0.38110238327173099531e-3) * t23272 - F::new(0.1270341277572436651e-3) * t19153 - t19163 + t3174 * t2888 * t32143 * t824 / F::new(48.0) - t32150 / F::new(48.0) - t385 * t154 * t907 * t31086 / F::new(96.0) + F::new(77.0) / F::new(162.0) * t11383 * t909 - F::new(11.0) / F::new(36.0) * t3849 * t3181 + t1220 * t10038 / F::new(12.0) - t32164 / F::new(288.0) - F::new(11.0) / F::new(108.0) * t32166;
    t32168
}
