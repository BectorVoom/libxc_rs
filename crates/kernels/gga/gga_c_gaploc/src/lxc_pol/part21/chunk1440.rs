//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1440/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1440<F: Float>(t1890: F, t1966: F, t28813: F, t28816: F, t28820: F, t28822: F, t33690: F, t33692: F, t33695: F, t33702: F, t33705: F, t33708: F, t33711: F, t33713: F, t33716: F, t33722: F, t33728: F, t38907: F, t590: F) -> F {
    let t39268 = -t33690 - F::new(0.51123901271894332902e1) * t1966 * t1890 * t38907 * t590 + t33692 - t33695 + t33702 + t33705 - t33708 + t33711 + t33713 + t33716 - t33722 + t28813 - t28816 + t33728 + F::new(0.76685851907841499354e0) * t28820 - F::new(0.10224780254378866581e1) * t28822;
    t39268
}
