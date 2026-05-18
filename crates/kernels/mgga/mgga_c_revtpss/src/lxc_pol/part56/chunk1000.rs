//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1000/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1000<F: Float>(t32246: F, t32267: F, t32271: F, t32288: F, t33943: F, t33947: F, t33952: F, t33956: F, t33960: F, t33965: F, t33967: F, t33971: F, t8586: F, t8706: F) -> F {
    let t33973 = t32246 + F::new(0.57119737665102352616e0) * t33943 * t8586 - F::new(0.17135921299530705785e1) * t8706 * t33947 - F::new(0.11423947533020470523e1) * t8706 * t33952 + F::new(0.11423947533020470523e1) * t8706 * t33956 + t32267 - t32271 - F::new(0.1859366460452550541e-3) * t33960 + F::new(0.3718732920905101082e-3) * t33965 + F::new(0.3718732920905101082e-3) * t33967 + t32288 + F::new(0.7437465841810202164e-3) * t33971;
    t33973
}
