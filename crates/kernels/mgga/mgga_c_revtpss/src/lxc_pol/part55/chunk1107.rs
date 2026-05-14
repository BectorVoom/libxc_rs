//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1107/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1107<F: Float>(t109731: F, t122407: F, t125771: F, t125775: F, t128694: F, t27845: F, t27852: F, t27857: F, t27865: F, t27896: F, t27973: F, t27981: F, t28911: F, t32690: F, t32700: F, t32719: F, t34237: F, t7303: F) -> (F,) {
    let t128767 = -0.17135921299530705785e1 * t32700 * t34237 + 0.17347256376410398924e1 * t32690 * t27896 - 0.56468933516960933999e-3 * t125771 - 0.11423947533020470523e1 * t32719 * t109731 * t7303 - 0.11423947533020470523e1 * t32719 * t28911 * t27852 - 0.11423947533020470523e1 * t32719 * t28911 * t27857 - 0.17347256376410398924e1 * t128694 * t27981 - 0.17347256376410398924e1 * t122407 * t27973 - 0.11423947533020470523e1 * t32719 * t28911 * t27845 - 0.17347256376410398924e1 * t122407 * t27865 + 0.7437465841810202164e-3 * t125775;
    (t128767,)
}
