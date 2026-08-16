//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1188/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1188<F: Float>(t2030: F, t20559: F, t8919: F, t301: F, t31146: F, t4256: F, t9529: F, t37791: F, t37792: F, t40330: F, t40332: F, t40336: F, t40340: F, t40344: F, t40347: F, t40350: F, t40354: F, t40358: F, t40361: F, t40365: F, t40369: F, t40371: F, t40374: F) -> F {
    let t40377 = t2030 * t20559 * t8919;
    let t40381 = t31146 * t4256 * t9529 * t301;
    let t40383 = F::cast_from(0.20007875121765877254e-2_f64) * t40330 - F::cast_from(0.40015750243531754507e-2_f64) * t40332 - t37791 - t37792 + F::cast_from(0.68765625e-1_f64) * t40336 + F::cast_from(0.916875e-1_f64) * t40340 - F::cast_from(0.4584375e-1_f64) * t40344 - t40347 / F::cast_from(32.0_f64) - t40350 / F::cast_from(16.0_f64) + F::cast_from(0.916875e-1_f64) * t40354 - F::cast_from(0.916875e-1_f64) * t40358 - F::cast_from(0.4584375e-1_f64) * t40361 - F::cast_from(0.4584375e-1_f64) * t40365 - F::cast_from(0.4584375e-1_f64) * t40369 + F::cast_from(0.16809375e0_f64) * t40371 - F::cast_from(0.916875e-1_f64) * t40374 - F::cast_from(0.4584375e-1_f64) * t40377 + F::cast_from(0.22921875e0_f64) * t40381;
    t40383
}
