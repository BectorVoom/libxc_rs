//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1319/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1319(t33650: f64, t10857: f64, t5676: f64, t10820: f64, t15362: f64, t6066: f64, t1029: f64, t7344: f64, t7803: f64, t20671: f64, t22543: f64, t25359: f64) -> (f64, f64, f64, f64, f64) {
    let t33651 = 0.14896037479937677779e-1_f64 * t33650;
    let t33652 = t5676 * t10857;
    let t33653 = 0.29792074959875355558e-1_f64 * t33652;
    let t33656 = 0.85801175884441024006e1_f64 * t15362 * t6066 * t10820;
    let t33658 = t7803 * t1029 * t7344;
    let t33659 = 0.19171462976960374838e0_f64 * t33658;
    let t33661 = t22543 * t20671 * t25359;
    (t33651, t33653, t33656, t33659, t33661)
}
