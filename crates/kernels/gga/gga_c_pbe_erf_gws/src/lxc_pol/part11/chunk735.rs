//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 735/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk735<F: Float>(t11020: F, t11023: F, t12323: F, t225: F, t11026: F, t11038: F, t12497: F, t1714: F, t12501: F, t12505: F, t657: F, t12509: F, t10519: F, t10521: F, t10581: F, t10583: F, t10585: F, t12495: F, t12515: F, t25: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12827 = 32.0 / 45.0 * t11020;
    let t12828 = 16.0 / 45.0 * t11023;
    let t12829 = t12323 * t225;
    let t12832 = 4.0 / 15.0 * t11026;
    let t12834 = 8.0 / 45.0 * t11038;
    let t12837 = t1714 * t12497;
    let t12840 = t1714 * t12501;
    let t12843 = t657 * t12505;
    let t12846 = t657 * t12509;
    let t12854 = -0.39990740740740740742e-1 * t12495 - 0.35991666666666666667e-1 * t12515 + 0.13333333333333333333e-1 * t25 * t12837 - 0.66666666666666666666e-2 * t25 * t12840 - 0.39999999999999999999e-1 * t25 * t12843 + 0.39999999999999999999e-1 * t25 * t12846 - 0.26666666666666666667e-1 * t10519 + 0.13333333333333333334e-1 * t10521 + 0.35991666666666666666e-1 * t10585 + 0.23994444444444444444e-1 * t10581 - 0.71983333333333333333e-1 * t10583;
    (t12827, t12828, t12829, t12832, t12834, t12837, t12840, t12843, t12846, t12854)
}
