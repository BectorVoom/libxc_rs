//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1174/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1174<F: Float>(t1029: F, t23099: F, t7396: F, t10811: F, t28837: F, t2021: F, t7372: F, t8520: F, t11050: F, t1986: F, t28793: F, t28796: F, t28800: F, t28810: F, t33666: F, t33668: F, t33671: F, t33673: F, t33675: F, t33676: F, t33683: F, t33685: F, t5662: F, t590: F) -> (F,) {
    let t33689 = t23099 * t1029 * t7396;
    let t33690 = 0.38342925953920749676e0 * t33689;
    let t33691 = t10811 * t28837;
    let t33692 = 0.17875244975925213335e0 * t33691;
    let t33694 = t2021 * t8520 * t7372;
    let t33695 = 0.29792074959875355558e-1 * t33694;
    let t33696 = t33666 + t33668 + t33671 - t33673 - t33675 - 0.1022478025437886658e1 * t1986 * t33676 * t590 + t33683 - t33685 + t28793 + t28796 + t28800 - 0.51123901271894332905e0 * t5662 * t11050 - t28810 - t33690 + t33692 - t33695;
    (t33696,)
}
