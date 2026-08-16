//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2786/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2786(t14114: f64, t14216: f64, t14145: f64, t2482: f64, t4114: f64, t6843: f64, t1432: f64, t22379: f64, t2470: f64, t1437: f64, t4104: f64, t6861: f64) -> (f64, f64, f64, f64) {
    let t74862 = t14114 * t14216;
    let t74866 = t2482 * t4114 * t6843 * t14145;
    let t74873 = t1432 * t22379 * t2470;
    let t74880 = t2482 * t1437 * t6861 * t4104;
    (t74862, t74866, t74873, t74880)
}
