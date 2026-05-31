//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1284/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1284<F: Float>(t1804: F, t1806: F, t20129: F, t20132: F, t23674: F, t23684: F, t23688: F, t23696: F, t23701: F, t23706: F, t23726: F, t23737: F, t23740: F, t23743: F, t27728: F, t27759: F, t27766: F, t27770: F, t27777: F, t27789: F, t2970: F, t457: F, t559: F, t7837: F, t7866: F, t7868: F, t9839: F) -> F {
    let t27797 = -t2970 * t27759 * t7837 / F::cast_from(6.0_f64) + t20129 / F::cast_from(216.0_f64) + t20132 / F::cast_from(288.0_f64) - t27766 / F::cast_from(72.0_f64) - t1804 * t27770 * t1806 * t559 * t457 / F::cast_from(6.0_f64) + t27777 / F::cast_from(216.0_f64) + t23674 / F::cast_from(54.0_f64) + t23684 / F::cast_from(24.0_f64) - t23688 / F::cast_from(36.0_f64) - t23696 / F::cast_from(72.0_f64) - F::cast_from(7.0_f64) / F::cast_from(216.0_f64) * t23701 - t23726 / F::cast_from(72.0_f64) - t23737 / F::cast_from(96.0_f64) - t23740 / F::cast_from(48.0_f64) - t23743 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t27789 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t7866 * t23706 * t9839 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t7866 * t7868 * t27728;
    t27797
}
