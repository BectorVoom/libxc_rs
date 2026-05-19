//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1226/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1226<F: Float>(t37835: F, t37838: F, t38528: F, t38532: F, t39679: F, t39721: F, t39723: F, t39738: F, t43215: F, t43217: F, t43219: F, t43225: F) -> F {
    let t44288 = -t39679 + F::cast_from(0.87327386630866483588e-2_f64) * t43215 - F::cast_from(0.97574405393827830187e-2_f64) * t43217 - F::cast_from(0.11565819519348392138e-2_f64) * t39721 + F::cast_from(0.32524801797942610062e-3_f64) * t39723 - F::cast_from(0.26198215989259945076e-1_f64) * t43219 + t38528 + t38532 + F::cast_from(0.58544643236296698113e-1_f64) * t37835 + F::cast_from(0.45022119329691164871e0_f64) * t37838 + t39738 + F::cast_from(0.69345773920434148507e0_f64) * t43225;
    t44288
}
