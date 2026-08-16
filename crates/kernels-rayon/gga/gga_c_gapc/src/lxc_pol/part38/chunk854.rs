//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 854/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk854(t9932: f64, t9934: f64, t3434: f64, t949: f64, t2749: f64, t3348: f64, t3322: f64, t9414: f64, t9898: f64, t9901: f64, t9904: f64, t9908: f64, t9910: f64, t9914: f64, t9917: f64, t9924: f64, t9930: f64) -> f64 {
    let t9935 = t9932 * t9934;
    let t9937 = t3434 * t949;
    let t9939 = t3348 * t2749;
    let t9941 = t9414 * t3322;
    let t9943 = 0.12890821708151275006e-8_f64 * t9898 + 0.21135226489492151266e-6_f64 * t9901 + 0.61900849231692170544e-6_f64 * t9904 - 0.42205124476153752644e-7_f64 * t9908 - 0.84410248952307505288e-7_f64 * t9910 - 0.42205124476153752644e-7_f64 * t9914 - 0.50027140879067581468e-8_f64 * t9917 + 0.10005428175813516294e-7_f64 * t9924 - 0.72956247115306889641e-9_f64 * t9930 + 0.24619655944423022376e-7_f64 * t9935 + 0.10821235962619981449e-3_f64 * t9937 + 0.11594181388521408695e-4_f64 * t9939 + 0.40021712703254065174e-7_f64 * t9941;
    t9943
}
