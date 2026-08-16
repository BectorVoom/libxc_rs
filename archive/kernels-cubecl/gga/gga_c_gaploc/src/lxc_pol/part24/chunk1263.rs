//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1263/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1263<F: Float>(t11054: F, t28073: F, t2925: F, t5241: F, t2679: F, t9805: F, t11053: F, t7383: F, t10627: F, t22623: F, t15482: F, t22622: F) -> (F, F, F, F) {
    let t32838 = t28073 * t11054;
    let t32839 = F::cast_from(0.11502877786176224903e1_f64) * t32838;
    let t32840 = t5241 * t2925;
    let t32842 = t9805 * t32840 * t2679;
    let t32843 = F::cast_from(0.11502877786176224903e1_f64) * t32842;
    let t32845 = t9805 * t11053 * t7383;
    let t32846 = F::cast_from(0.57514388930881124514e0_f64) * t32845;
    let t32847 = t22623 * t10627;
    let t32850 = F::cast_from(0.34082600847929555269e0_f64) * t22622 * t15482 * t32847;
    (t32839, t32843, t32846, t32850)
}
