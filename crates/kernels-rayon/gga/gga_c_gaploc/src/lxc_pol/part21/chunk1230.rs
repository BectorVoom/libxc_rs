//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1230/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1230(t32514: f64, t7572: f64, t7573: f64, t11054: f64, t28073: f64, t2925: f64, t5241: f64, t2679: f64, t9805: f64, t11053: f64, t7383: f64, t10627: f64, t22623: f64) -> (f64, f64, f64, f64, f64) {
    let t32835 = 0.12423108009070322895e3_f64 * t7572 * t7573 * t32514;
    let t32838 = t28073 * t11054;
    let t32839 = 0.11502877786176224903e1_f64 * t32838;
    let t32840 = t5241 * t2925;
    let t32842 = t9805 * t32840 * t2679;
    let t32843 = 0.11502877786176224903e1_f64 * t32842;
    let t32845 = t9805 * t11053 * t7383;
    let t32846 = 0.57514388930881124514e0_f64 * t32845;
    let t32847 = t22623 * t10627;
    (t32835, t32839, t32843, t32846, t32847)
}
