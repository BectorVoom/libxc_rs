//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1241/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1241(t1256: f64, t1271: f64, t13204: f64, t127: f64, t151: f64, t2124: f64, t2126: f64, t2168: f64, t23050: f64, t3467: f64, t3501: f64, t48806: f64, t48808: f64, t48810: f64, t48812: f64, t5: f64, t56111: f64, t56118: f64, t56132: f64, t56140: f64, t56148: f64, t56154: f64, t56159: f64, t56344: f64, t673: f64, t675: f64, t9955: f64) -> (f64, f64) {
    let t56467 = t13204 * t1256 * t1271;
    let t56489 = -0.60456845350037036744e-1_f64 * t2168 * t56154 + 0.10431793787746509425e1_f64 * t2124 * t2126 * t56118 + 0.18137053605011111023e0_f64 * t3501 * t56111 - 0.417271751509860377e1_f64 * t3467 * t2126 * t56467 - 0.86931614897887578546e-1_f64 * t673 * t675 * t5 * t56344 * t127 - 0.25391875047015555432e1_f64 * t48806 - 0.33855833396020740576e1_f64 * t48808 + 0.10156750018806222173e2_f64 * t48810 + 0.16927916698010370288e2_f64 * t48812 + t23050 - 0.52158968938732547128e0_f64 * t2124 * t151 * t56148 + 0.81616741222549999602e0_f64 * t3501 * t56140 + 0.24182738140014814697e0_f64 * t2168 * t56159 + 0.29019285768017777637e1_f64 * t9955 * t56132;
    (t56467, t56489)
}
