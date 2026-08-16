//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1379/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1379(t10110: f64, t1528: f64, t1902: f64, t1912: f64, t20936: f64, t25188: f64, t259: f64, t2718: f64, t28311: f64, t4268: f64, t5636: f64, t5637: f64, t5657: f64, t67339: f64, t7537: f64, t855: f64, t87898: f64, t87915: f64, t99010: f64, t99022: f64, t99036: f64) -> f64 {
    let t105723 = -0.12337005501361698274e-1_f64 * t99022 - 3.0_f64 * t99010 * t1528 - 0.78134368175290755733e-1_f64 * t87898 - 0.24674011002723396547e-1_f64 * t87915 - 3.0_f64 * t67339 * t1912 + 6.0_f64 * t855 * t2718 * t7537 * t5657 + 6.0_f64 * t25188 * t5637 + t20936 * t1902 * t259 + 0.49348022005446793095e-1_f64 * t99036 - 18.0_f64 * t855 * t10110 * t7537 * t5636 - 18.0_f64 * t4268 * t28311;
    t105723
}
