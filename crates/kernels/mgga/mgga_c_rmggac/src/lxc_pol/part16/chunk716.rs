//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 716/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk716<F: Float>(t2471: F, t558: F, t1743: F, t699: F, t8125: F, t8710: F, t8714: F, t8716: F, t8718: F, t8735: F, t9874: F, t9878: F, t9880: F, t9882: F, t9886: F, t9890: F, t9892: F, t9894: F, t9897: F, t9899: F) -> (F, F, F) {
    let t10417 = t2471 * t558;
    let t10420 = t699 * t1743;
    let t10443 = F::cast_from(0.68186654135613354324e-2_f64) * t9874 - F::cast_from(0.90915538847484472432e-2_f64) * t9878 + F::cast_from(0.1814407727691612783e-3_f64) * t9880 - F::cast_from(0.21168090156402149135e-3_f64) * t9882 + F::cast_from(0.13637330827122670865e-1_f64) * t9886 + F::cast_from(0.45457769423742236216e-1_f64) * t9890 + F::cast_from(0.9072038638458063915e-3_f64) * t9892 + F::cast_from(0.16934472125121719308e-2_f64) * t9894 - F::cast_from(0.9676841214355268176e-3_f64) * t8710 + F::cast_from(0.11289648083414479539e-2_f64) * t8714 - F::cast_from(0.36366215538993788972e-1_f64) * t8716 + F::cast_from(0.48488287385325051964e-1_f64) * t8718 + F::cast_from(0.79656924630363488034e-2_f64) * t9897 + F::cast_from(0.11974241701863808564e0_f64) * t9899 + t8125 + F::cast_from(0.35403077613494883571e-2_f64) * t8735;
    (t10417, t10420, t10443)
}
