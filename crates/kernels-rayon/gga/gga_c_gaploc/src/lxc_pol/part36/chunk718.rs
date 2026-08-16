//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 718/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk718(t13157: f64, t1457: f64, t6060: f64, t1445: f64, t2087: f64, t13124: f64, t13127: f64, t13132: f64, t13134: f64, t13138: f64, t13140: f64, t13144: f64, t13147: f64, t13152: f64, t13156: f64) -> (f64, f64, f64) {
    let t13158 = t1457 * t13157;
    let t13160 = 0.21450293971110256001e1_f64 * t6060 * t13158;
    let t13161 = t1445 * t13157;
    let t13163 = 0.62115540045351614476e2_f64 * t2087 * t13161;
    let t13164 = t13124 - 0.92023022289409799224e1_f64 * t13127 - t13132 + 0.23005755572352449806e2_f64 * t13134 + t13138 + t13140 + t13144 - 0.29792074959875355558e-1_f64 * t13147 - t13152 + t13156 - t13160 - t13163;
    (t13158, t13161, t13164)
}
