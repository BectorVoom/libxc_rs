//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1241/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1241<F: Float>(t1256: F, t1271: F, t13204: F, t127: F, t151: F, t2124: F, t2126: F, t2168: F, t23050: F, t3467: F, t3501: F, t48806: F, t48808: F, t48810: F, t48812: F, t5: F, t56111: F, t56118: F, t56132: F, t56140: F, t56148: F, t56154: F, t56159: F, t56344: F, t673: F, t675: F, t9955: F) -> (F, F) {
    let t56467 = t13204 * t1256 * t1271;
    let t56489 = -F::cast_from(0.60456845350037036744e-1_f64) * t2168 * t56154 + F::cast_from(0.10431793787746509425e1_f64) * t2124 * t2126 * t56118 + F::cast_from(0.18137053605011111023e0_f64) * t3501 * t56111 - F::cast_from(0.417271751509860377e1_f64) * t3467 * t2126 * t56467 - F::cast_from(0.86931614897887578546e-1_f64) * t673 * t675 * t5 * t56344 * t127 - F::cast_from(0.25391875047015555432e1_f64) * t48806 - F::cast_from(0.33855833396020740576e1_f64) * t48808 + F::cast_from(0.10156750018806222173e2_f64) * t48810 + F::cast_from(0.16927916698010370288e2_f64) * t48812 + t23050 - F::cast_from(0.52158968938732547128e0_f64) * t2124 * t151 * t56148 + F::cast_from(0.81616741222549999602e0_f64) * t3501 * t56140 + F::cast_from(0.24182738140014814697e0_f64) * t2168 * t56159 + F::cast_from(0.29019285768017777637e1_f64) * t9955 * t56132;
    (t56467, t56489)
}
