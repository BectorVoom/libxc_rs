//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 893/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk893<F: Float>(t1665: F, t1671: F, t31885: F, t31891: F, t31903: F, t31934: F, t31950: F, t31992: F, t32000: F, t32003: F, t32010: F, t33797: F, t33800: F, t33804: F, t33808: F, t33812: F, t33817: F, t33822: F, t33827: F, t33832: F, t8517: F, t8524: F) -> (F,) {
    let t33835 = -0.17135921299530705785e1 * t31903 * t33797 + 0.57119737665102352616e0 * t33800 * t8517 - 0.17135921299530705785e1 * t31891 * t33804 + 0.11423947533020470523e1 * t31934 * t33808 + 0.11423947533020470523e1 * t31891 * t33812 - 0.5578099381357651623e-3 * t32003 * t33817 + 0.5578099381357651623e-3 * t32010 * t1665 - 0.1859366460452550541e-3 * t33822 * t8524 + 0.3718732920905101082e-3 * t31950 * t33827 - 0.3718732920905101082e-3 * t32000 * t1671 - t31885 - 0.12395776403017003607e-3 * t31992 * t33832;
    (t33835,)
}
