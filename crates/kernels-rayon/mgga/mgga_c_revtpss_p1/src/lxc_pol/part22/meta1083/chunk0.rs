//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3913/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3913(t13790: f64, t5658: f64, t10022: f64, t2782: f64, t1882: f64, t5710: f64, t4086: f64, t543: f64, t74973: f64, t1399: f64, t22009: f64, t3924: f64, t46522: f64, t47351: f64, t47352: f64, t47364: f64, t47381: f64, t47389: f64, t47391: f64, t49283: f64, t49289: f64, t49296: f64, t5755: f64, t6862: f64, t820: f64) -> (f64, f64) {
    let t75188 = t13790 * t5658;
    let t75190 = t2782 * t10022 * t75188;
    let t75198 = t5710 * t1882;
    let t75205 = t2782 * t4086 * t74973 * t543;
    let t75209 = 0.13170898365871023197e1_f64 * t820 * t46522 * t6862 + 0.10975748638225852664e-1_f64 * t49283 - t47351 + 0.52039682876708176102e-2_f64 * t47352 - 0.43902994552903410656e-1_f64 * t75190 - 0.29268663035268940438e-1_f64 * t49289 + 0.21951497276451705328e-1_f64 * t49296 - 0.11565819519348392139e-2_f64 * t47364 - 0.65854491829355115987e0_f64 * t5755 * t22009 * t3924 - 0.26341796731742046394e1_f64 * t5755 * t75198 * t1399 - 0.22089088168956307394e-3_f64 * t47381 + 0.10975748638225852664e-1_f64 * t75205 - 0.34146773541147097178e-1_f64 * t47389 + 0.65049603595885220126e-3_f64 * t47391;
    (t75198, t75209)
}
