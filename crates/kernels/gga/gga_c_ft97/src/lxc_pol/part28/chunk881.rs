//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 881/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk881<F: Float>(t165: F, t34961: F, t28: F, t1058: F, t7340: F, t1360: F, t6723: F, t32722: F, t925: F, t1969: F, t1023: F, t1349: F, t32701: F, t32703: F, t32708: F, t32750: F, t34800: F, t34803: F, t34948: F, t34950: F, t34952: F, t34954: F, t34956: F, t5772: F, t6580: F, t6589: F, t7309: F, t7342: F, t7412: F) -> (F, F, F, F, F, F, F, F) {
    let t34962 = t34961 * t165;
    let t34963 = t28 * t34962;
    let t34966 = t7340 * t1058;
    let t34967 = t28 * t34966;
    let t34970 = t1360 * t6723;
    let t34971 = t28 * t34970;
    let t34974 = t32722 * t925;
    let t34975 = t1969 * t34974;
    let t34978 = -t32701 - t32703 - t32708 + t1349 * t34800 - F::new(2.0) / F::new(3.0) * t1349 * t34803 - t7309 * t6589 / F::new(3.0) - F::new(2.0) * t34948 + F::new(4.0) * t34950 - F::new(4.0) * t34952 - F::new(2.0) * t34954 - F::new(4.0) * t34956 - t1023 * t7412 + t6580 * t7342 / F::new(6.0) + t1349 * t34963 / F::new(6.0) + t1349 * t34967 / F::new(6.0) + t32750 + t1349 * t34971 / F::new(3.0) - t5772 * t34975 / F::new(18.0);
    (t34962, t34963, t34966, t34967, t34970, t34971, t34975, t34978)
}
