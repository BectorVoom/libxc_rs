//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 622/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk622(t158: f64, t3460: f64, t1054: f64, t1790: f64, t183: f64, t3410: f64, t1034: f64, t1044: f64, t164: f64, t167: f64, t1717: f64, t1721: f64, t3441: f64, t588: f64) -> (f64, f64, f64, f64) {
    let t3461 = t3460 * t158;
    let t3466 = t1054 * t1054;
    let t3467 = t1790 * t3466;
    let t3470 = t183 * t3410;
    let t3487 = 0.13170898365871023197e1_f64 * t1717 * t3470 * t1721 - 0.13170898365871023197e1_f64 * t588 * t1044 * t1034 * t164 - 0.65854491829355115987e0_f64 * t588 * t183 * t3441 * t164 - 0.65854491829355115987e0_f64 * t588 * t3470 * t164 + 0.65854491829355115987e0_f64 * t167 * t3460;
    (t3461, t3466, t3467, t3487)
}
