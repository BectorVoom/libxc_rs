//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 385/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk385(t1973: f64, t1974: f64, t1676: f64, t591: f64, t1683: f64, t1685: f64, t240: f64, t1642: f64, t1667: f64, t1671: f64, t1686: f64, t1961: f64, t1966: f64, t764: f64) -> (f64, f64, f64, f64, f64) {
    let t1975 = t1973 * t1974;
    let t1979 = t591 * t1676;
    let t1980 = t1683 * t1685;
    let t1987 = t240 * t591;
    let t1990 = -t1642 + t1667 + t240 * (-0.3109e-1_f64 * t1961 * t764 + 1.0_f64 * t1966 * t1975 + t1642 - t1667 - 0.19751789702565206229e-1_f64 * t1671 + 0.58482233974552040708e0_f64 * t1979 * t1980) + 0.19751789702565206229e-1_f64 * t240 * t1671 - 0.58482233974552040708e0_f64 * t1987 * t1686;
    (t1975, t1979, t1980, t1987, t1990)
}
