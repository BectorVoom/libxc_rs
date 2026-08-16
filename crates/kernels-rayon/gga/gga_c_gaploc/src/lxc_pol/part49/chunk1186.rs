//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1186/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1186(t13779: f64, t1407: f64, t12065: f64, t9285: f64, t447: f64, t46849: f64, t204: f64, t2476: f64, t40219: f64, t41909: f64, t41915: f64, t41919: f64, t41922: f64, t41927: f64, t41930: f64, t41933: f64, t41935: f64) -> (f64, f64) {
    let t47949 = t1407 * t13779;
    let t47951 = t9285 * t12065;
    let t47953 = t46849 * t447;
    let t47955 = t2476 * t204 * t47953;
    let t47960 = -0.19171462976960374838e0_f64 * t47949 + 0.35750489951850426669e0_f64 * t47951 + 0.46011511144704899612e1_f64 * t47955 + 0.14896037479937677779e-1_f64 * t41909 + t41915 + t41919 + t41922 + 0.76685851907841499354e0_f64 * t40219 - t41927 - t41930 + t41933 + 0.15337170381568299871e2_f64 * t41935;
    (t47953, t47960)
}
