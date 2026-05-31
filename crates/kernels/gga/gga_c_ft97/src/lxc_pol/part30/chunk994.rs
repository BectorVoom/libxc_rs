//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 994/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk994<F: Float>(t10002: F, t35639: F, t27915: F, t7437: F, t2568: F, t35678: F, t766: F, t1403: F, t35286: F, t681: F, t10157: F, t140707: F, t27836: F, t27894: F, t27958: F, t28467: F, t33255: F, t33575: F, t35282: F, t5996: F, t6002: F, t6003: F, t6745: F, t7491: F) -> (F, F, F) {
    let t149997 = t10002 * t35639;
    let t150009 = t7437 * t27915;
    let t150014 = t2568 * t35678 * t766;
    let t150017 = t1403 * t681 * t35286;
    let t150020 = F::cast_from(2.0_f64) * t6002 * t10157 * t6003 * t27836 + F::cast_from(4.0_f64) * t149997 - t7437 * t27958 / F::cast_from(3.0_f64) - t7437 * t28467 / F::cast_from(3.0_f64) + t5996 * t35282 / F::cast_from(6.0_f64) - t6745 * t33255 / F::cast_from(3.0_f64) - t6745 * t33575 / F::cast_from(3.0_f64) - t150009 / F::cast_from(18.0_f64) + t27894 * t7491 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) * t150014 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t150017 + t140707 / F::cast_from(9.0_f64);
    (t149997, t150014, t150020)
}
