//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1375/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1375<F: Float>(t114770: F, t114773: F, t114775: F, t114779: F, t114783: F, t114785: F, t114787: F, t114790: F, t114794: F, t114803: F, t114807: F, t114814: F, t114816: F, t114823: F, t116848: F, t116861: F, t1843: F, t1911: F, t2163: F, t22747: F, t30716: F, t30959: F, t508: F, t569: F, t5877: F, t8233: F) -> F {
    let t116865 = -t116848 * t508 + t116861 * t569 - F::new(3.0) * t1843 * t30716 + F::new(3.0) * t1911 * t30959 - t2163 * t22747 - F::new(3.0) * t5877 * t8233 + t114770 - t114773 + t114775 + t114779 + t114783 - t114785 - t114787 - t114790 + t114794 - t114803 + t114807 - t114814 - t114816 - t114823;
    t116865
}
