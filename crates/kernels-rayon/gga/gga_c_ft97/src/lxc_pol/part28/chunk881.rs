//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 881/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk881(t165: f64, t34961: f64, t28: f64, t1058: f64, t7340: f64, t1360: f64, t6723: f64, t32722: f64, t925: f64, t1969: f64, t1023: f64, t1349: f64, t32701: f64, t32703: f64, t32708: f64, t32750: f64, t34800: f64, t34803: f64, t34948: f64, t34950: f64, t34952: f64, t34954: f64, t34956: f64, t5772: f64, t6580: f64, t6589: f64, t7309: f64, t7342: f64, t7412: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34962 = t34961 * t165;
    let t34963 = t28 * t34962;
    let t34966 = t7340 * t1058;
    let t34967 = t28 * t34966;
    let t34970 = t1360 * t6723;
    let t34971 = t28 * t34970;
    let t34974 = t32722 * t925;
    let t34975 = t1969 * t34974;
    let t34978 = -t32701 - t32703 - t32708 + t1349 * t34800 - 2.0_f64 / 3.0_f64 * t1349 * t34803 - t7309 * t6589 / 3.0_f64 - 2.0_f64 * t34948 + 4.0_f64 * t34950 - 4.0_f64 * t34952 - 2.0_f64 * t34954 - 4.0_f64 * t34956 - t1023 * t7412 + t6580 * t7342 / 6.0_f64 + t1349 * t34963 / 6.0_f64 + t1349 * t34967 / 6.0_f64 + t32750 + t1349 * t34971 / 3.0_f64 - t5772 * t34975 / 18.0_f64;
    (t34962, t34963, t34966, t34967, t34970, t34971, t34975, t34978)
}
