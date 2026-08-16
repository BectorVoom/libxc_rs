//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1106/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1106(t10037: f64, t10042: f64, t10047: f64, t10052: f64, t10055: f64, t34649: f64, t7771: f64, t9490: f64, t9491: f64, t9492: f64, t9493: f64, t10383: f64, t10384: f64, t42296: f64, t42297: f64, t42298: f64, t42299: f64, t42300: f64, t42301: f64, t42306: f64, t9600: f64, t9601: f64) -> (f64, f64) {
    let t48064 = t34649 - t9490 + t9491 + t9492 - t9493 - t7771 - t10037 + t10042 + t10047 + t10052 - t10055;
    let t48067 = t42296 - t42297 + t42298 + t9600 + t9601 + t42299 + t42300 - t42301 + t42306 + t10383 - t10384;
    (t48064, t48067)
}
