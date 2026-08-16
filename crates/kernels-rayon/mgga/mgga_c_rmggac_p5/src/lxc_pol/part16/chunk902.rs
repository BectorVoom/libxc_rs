//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 902/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk902(t3351: f64, t3352: f64, t6387: f64, t880: f64, t2144: f64, t6530: f64, t1929: f64, t1986: f64, t7720: f64, t495: f64, t515: f64, t6522: f64, t7230: f64) -> (f64, f64, f64, f64) {
    let t44990 = t3351 * t3352 * t880 * t6387;
    let t44994 = t3351 * t3352 * t2144 * t6530;
    let t44996 = t1986 * t1929;
    let t44997 = t7720 * t44996;
    let t45002 = t7230 * t3352 * t515 * t6522 * t495;
    (t44990, t44994, t44997, t45002)
}
