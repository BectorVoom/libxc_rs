//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1432/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1432<F: Float>(t33680: F, t33687: F, t33692: F, t36659: F, t36660: F, t36661: F, t36662: F, t36664: F, t36666: F, t36668: F, t36669: F, t33719: F, t36671: F, t36672: F, t36673: F, t36674: F, t36675: F, t36676: F, t36678: F, t36679: F, t36680: F, t36681: F) -> (F, F) {
    let t38753 = -t36659 + t36660 - t36661 + t36662 - F::new(0.2445773654513888889e-4) * t33680 + t36664 - F::new(0.2445773654513888889e-4) * t33687 + t36666 + F::new(0.56912804804009946682e-7) * t33692 - t36668 + t36669;
    let t38755 = -t36671 - t36672 - t36673 - t36674 + t36675 - t36676 + F::new(0.12650553385416666667e-5) * t33719 + t36678 - t36679 - t36680 + t36681;
    (t38753, t38755)
}
