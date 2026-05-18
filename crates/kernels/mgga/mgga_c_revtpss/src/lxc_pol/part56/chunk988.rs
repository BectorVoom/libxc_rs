//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 988/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk988<F: Float>(t31748: F, t31751: F, t31764: F, t31783: F, t31786: F, t31787: F, t31794: F, t31803: F, t33675: F, t33679: F, t33683: F, t33688: F, t33691: F, t7779: F, t8472: F) -> F {
    let t33694 = -t31748 + t31751 - F::new(0.28234466758480466999e-3) * t33675 - t31764 - F::new(0.112937867033921868e-2) * t33679 - F::new(0.28234466758480466999e-3) * t33683 + t31783 - t31786 - F::new(0.17347256376410398924e1) * t31787 * t7779 + F::new(0.17347256376410398924e1) * t8472 * t33688 + F::new(0.8673628188205199462e0) * t31794 * t33691 - t31803;
    t33694
}
