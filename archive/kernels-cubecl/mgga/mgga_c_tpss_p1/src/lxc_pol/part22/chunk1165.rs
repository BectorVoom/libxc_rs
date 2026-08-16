//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1165/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1165<F: Float>(t1629: F, t3234: F, t762: F, t10161: F, t10166: F, t1213: F, t1244: F, t12996: F, t13000: F, t13004: F, t13006: F, t13009: F, t13013: F, t13015: F, t13018: F, t13021: F, t13023: F, t3244: F, t4413: F) -> F {
    let t13027 = t762 * t1629 * t3234;
    let t13030 = -t1213 * t12996 / F::cast_from(48.0_f64) + t4413 * t13000 / F::cast_from(1536.0_f64) - t13004 + t13006 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t10161 - t10166 - t4413 * t13009 / F::cast_from(384.0_f64) + t13013 - t1244 * t13015 / F::cast_from(768.0_f64) - F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t13018 - t13021 + t3244 * t13023 / F::cast_from(8.0_f64) + t3244 * t13027 / F::cast_from(16.0_f64);
    t13030
}
