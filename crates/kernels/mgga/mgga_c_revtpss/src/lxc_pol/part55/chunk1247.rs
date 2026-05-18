//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1247/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1247<F: Float>(t1032: F, t8085: F, t1955: F, t120988: F, t120995: F, t122309: F, t122312: F, t122315: F, t122319: F, t125599: F, t125603: F, t28003: F, t32250: F, t32690: F, t5774: F, t7298: F, t8706: F, t8708: F) -> (F, F, F) {
    let t128617 = t8085 * t1032;
    let t128618 = t1955 * t128617;
    let t128625 = -t120988 + F::new(0.17347256376410398924e1) * t32690 * t28003 + F::new(0.7437465841810202164e-3) * t125599 - t122309 + F::new(0.28559868832551176308e-1) * t122312 - F::new(0.50779446784275991476e-1) * t122315 - F::new(0.14874931683620404328e-2) * t125603 + t122319 + F::new(0.17347256376410398924e1) * t128618 * t7298 - F::new(0.17135921299530705785e1) * t8706 * t32250 * t8708 * t5774 + t120995;
    (t128617, t128618, t128625)
}
