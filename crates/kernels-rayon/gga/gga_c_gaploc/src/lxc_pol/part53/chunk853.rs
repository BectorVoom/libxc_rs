//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 853/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk853(t40377: f64, t40392: f64, t40395: f64, t2890: f64, t9267: f64, t9278: f64, t20671: f64, t31047: f64, t34814: f64, t26984: f64, t9294: f64, t1424: f64, t2875: f64, t544: f64, t9065: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42170 = 0.19171462976960374838e0_f64 * t40377;
    let t42172 = 0.15337170381568299871e1_f64 * t40392;
    let t42173 = 0.29792074959875355558e-1_f64 * t40395;
    let t42183 = t9267 * t2890 * t9278;
    let t42184 = 0.19171462976960374838e1_f64 * t42183;
    let t42187 = t31047 * t20671 * t34814;
    let t42188 = 0.42603251059911944084e0_f64 * t42187;
    let t42189 = t26984 * t9294;
    let t42194 = 0.39722766613167140743e-1_f64 * t544 * t9065 * t2875 * t1424;
    (t42170, t42172, t42173, t42184, t42188, t42189, t42194)
}
