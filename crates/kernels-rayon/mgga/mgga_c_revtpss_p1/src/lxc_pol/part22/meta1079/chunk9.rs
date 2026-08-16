//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3879/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3879(t46918: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t48917: f64, t48922: f64, t48929: f64, t48937: f64, t48941: f64, t22041: f64, t3957: f64) -> (f64, f64) {
    let t74542 = -0.10164000561857065645e-3_f64 * t48917 + 0.2032800112371413129e-3_f64 * t48922 + 0.45351183609335988442e-1_f64 * t46918 - 0.25410001404642664112e-4_f64 * t48929 - 0.50820002809285328224e-5_f64 * t46931 + 0.25410001404642664112e-5_f64 * t46934 + 0.25410001404642664112e-5_f64 * t46941 + 0.9035796499490931358e-4_f64 * t46944 - 0.91476005056713590802e-4_f64 * t46947 - 0.16006300097412701803e-1_f64 * t48937 + 0.10164000561857065645e-3_f64 * t48941;
    let t74547 = t3957 * t22041;
    (t74542, t74547)
}
