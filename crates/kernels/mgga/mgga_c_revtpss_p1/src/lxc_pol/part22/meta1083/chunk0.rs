//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3913/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3913<F: Float>(t13790: F, t5658: F, t10022: F, t2782: F, t1882: F, t5710: F, t4086: F, t543: F, t74973: F, t1399: F, t22009: F, t3924: F, t46522: F, t47351: F, t47352: F, t47364: F, t47381: F, t47389: F, t47391: F, t49283: F, t49289: F, t49296: F, t5755: F, t6862: F, t820: F) -> (F, F) {
    let t75188 = t13790 * t5658;
    let t75190 = t2782 * t10022 * t75188;
    let t75198 = t5710 * t1882;
    let t75205 = t2782 * t4086 * t74973 * t543;
    let t75209 = F::cast_from(0.13170898365871023197e1_f64) * t820 * t46522 * t6862 + F::cast_from(0.10975748638225852664e-1_f64) * t49283 - t47351 + F::cast_from(0.52039682876708176102e-2_f64) * t47352 - F::cast_from(0.43902994552903410656e-1_f64) * t75190 - F::cast_from(0.29268663035268940438e-1_f64) * t49289 + F::cast_from(0.21951497276451705328e-1_f64) * t49296 - F::cast_from(0.11565819519348392139e-2_f64) * t47364 - F::cast_from(0.65854491829355115987e0_f64) * t5755 * t22009 * t3924 - F::cast_from(0.26341796731742046394e1_f64) * t5755 * t75198 * t1399 - F::cast_from(0.22089088168956307394e-3_f64) * t47381 + F::cast_from(0.10975748638225852664e-1_f64) * t75205 - F::cast_from(0.34146773541147097178e-1_f64) * t47389 + F::cast_from(0.65049603595885220126e-3_f64) * t47391;
    (t75198, t75209)
}
