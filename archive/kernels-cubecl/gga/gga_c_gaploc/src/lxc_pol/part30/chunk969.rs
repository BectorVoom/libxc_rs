//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 969/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk969<F: Float>(t10544: F, t6514: F, t986: F, t544: F, t2386: F, t2389: F, t2898: F, t10314: F, t204: F, t2476: F, t594: F, t1: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10545 = F::cast_from(0.25561950635947166451e0_f64) * t10544;
    let t10546 = t6514 * t986;
    let t10547 = t544 * t10546;
    let t10549 = F::cast_from(0.25025342966295298669e1_f64) * t10547 * t2386;
    let t10550 = t2898 * t2389;
    let t10551 = F::cast_from(0.29792074959875355558e-1_f64) * t10550;
    let t10552 = t204 * t10314;
    let t10554 = F::cast_from(0.46011511144704899612e1_f64) * t2476 * t10552;
    let t10555 = t594 * t986;
    let t10556 = t10555 * t1;
    (t10545, t10546, t10547, t10549, t10551, t10552, t10554, t10555, t10556)
}
